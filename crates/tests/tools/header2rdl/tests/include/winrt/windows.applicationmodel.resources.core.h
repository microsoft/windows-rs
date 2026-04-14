
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
#ifndef __windows2Eapplicationmodel2Eresources2Ecore_h__
#define __windows2Eapplicationmodel2Eresources2Ecore_h__
#ifndef __windows2Eapplicationmodel2Eresources2Ecore_p_h__
#define __windows2Eapplicationmodel2Eresources2Ecore_p_h__


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
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface INamedResource;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource ABI::Windows::ApplicationModel::Resources::Core::INamedResource

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceCandidate;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceCandidate2;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2 ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate2

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceCandidate3;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3 ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate3

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceContext;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext ABI::Windows::ApplicationModel::Resources::Core::IResourceContext

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceContextStatics;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics ABI::Windows::ApplicationModel::Resources::Core::IResourceContextStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceContextStatics2;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2 ABI::Windows::ApplicationModel::Resources::Core::IResourceContextStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceContextStatics3;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3 ABI::Windows::ApplicationModel::Resources::Core::IResourceContextStatics3

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceContextStatics4;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4 ABI::Windows::ApplicationModel::Resources::Core::IResourceContextStatics4

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceManager;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager ABI::Windows::ApplicationModel::Resources::Core::IResourceManager

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceManager2;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2 ABI::Windows::ApplicationModel::Resources::Core::IResourceManager2

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceManagerStatics;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics ABI::Windows::ApplicationModel::Resources::Core::IResourceManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceMap;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap ABI::Windows::ApplicationModel::Resources::Core::IResourceMap

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    interface IResourceQualifier;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier ABI::Windows::ApplicationModel::Resources::Core::IResourceQualifier

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("430ecece-1418-5d19-81b2-5ddb381603cc"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("398c4183-793d-5b00-819b-4aef92485e94"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

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
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    class NamedResource;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0c5a605f-a7f1-5030-a179-9fd363caf3b5"))
IIterator<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*, ABI::Windows::ApplicationModel::Resources::Core::INamedResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Resources.Core.NamedResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_t;
#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e80d3d9d-96c9-579e-8e42-d550700de925"))
IIterable<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*, ABI::Windows::ApplicationModel::Resources::Core::INamedResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Resources.Core.NamedResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_t;
#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    class ResourceCandidate;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e151bd8c-a286-57ab-bcea-79b7bc2687a1"))
IIterator<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*, ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Resources.Core.ResourceCandidate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*> __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_t;
#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d89c9498-8869-57f8-a883-9c2dfeecb6c6"))
IIterable<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*, ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Resources.Core.ResourceCandidate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*> __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_t;
#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    class ResourceMap;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3dedcae6-d048-5eaa-afa2-fb4a7970ef68"))
IIterator<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*, ABI::Windows::ApplicationModel::Resources::Core::IResourceMap*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Resources.Core.ResourceMap>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_t;
#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8bbe1154-19aa-53e7-9d6e-dc7d358580f4"))
IIterable<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*, ABI::Windows::ApplicationModel::Resources::Core::IResourceMap*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Resources.Core.ResourceMap>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_t;
#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    class ResourceQualifier;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fedb74db-e74f-597a-b9bf-704b0dadca38"))
IIterator<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*, ABI::Windows::ApplicationModel::Resources::Core::IResourceQualifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Resources.Core.ResourceQualifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*> __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_t;
#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2eee68e2-687c-5f7a-a14e-588c4d3089e1"))
IIterable<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*, ABI::Windows::ApplicationModel::Resources::Core::IResourceQualifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Resources.Core.ResourceQualifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*> __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_t;
#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bd4b0143-3a22-5ee2-92ed-7bc3c129e52b"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*, ABI::Windows::ApplicationModel::Resources::Core::INamedResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7fdcc3d7-e13e-5f76-afc6-0769c4086399"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bf16482e-80ed-51f0-b9c9-3a804e2d6403"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("159782ba-798d-5ae6-85c7-c7bb551aae61"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*, ABI::Windows::ApplicationModel::Resources::Core::IResourceMap*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("28cf8d5e-69f6-59f6-9865-bca97d59f74f"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9895431e-a817-5509-91e6-d90a933ed830"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CIStorageFile_USE
#define DEF___FIIterator_1_Windows__CStorage__CIStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("314d2318-74ee-535c-b361-2144dbc573a0"))
IIterator<ABI::Windows::Storage::IStorageFile*> : IIterator_impl<ABI::Windows::Storage::IStorageFile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.IStorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::IStorageFile*> __FIIterator_1_Windows__CStorage__CIStorageFile_t;
#define __FIIterator_1_Windows__CStorage__CIStorageFile ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CIStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CIStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CIStorageFile_USE
#define DEF___FIIterable_1_Windows__CStorage__CIStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("76d43c7e-fd09-5908-a2b9-a49b4848294b"))
IIterable<ABI::Windows::Storage::IStorageFile*> : IIterable_impl<ABI::Windows::Storage::IStorageFile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.IStorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::IStorageFile*> __FIIterable_1_Windows__CStorage__CIStorageFile_t;
#define __FIIterable_1_Windows__CStorage__CIStorageFile ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CIStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CIStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapChangedEventArgs_1_HSTRING_USE
#define DEF___FIMapChangedEventArgs_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60141efb-f2f9-5377-96fd-f8c60d9558b5"))
IMapChangedEventArgs<HSTRING> : IMapChangedEventArgs_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapChangedEventArgs`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapChangedEventArgs<HSTRING> __FIMapChangedEventArgs_1_HSTRING_t;
#define __FIMapChangedEventArgs_1_HSTRING ABI::Windows::Foundation::Collections::__FIMapChangedEventArgs_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapChangedEventArgs_1_HSTRING_USE */



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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4825d6c4-835a-5da1-9bdd-12e97e16fb7a"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*, ABI::Windows::ApplicationModel::Resources::Core::INamedResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d4349ade-93b1-5325-ba5c-05f35eeffc55"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*, ABI::Windows::ApplicationModel::Resources::Core::IResourceMap*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FMapChangedEventHandler_2_HSTRING_HSTRING_USE
#define DEF___FMapChangedEventHandler_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2663f37-2e1b-500c-ad68-c3ed7a8f74c8"))
MapChangedEventHandler<HSTRING, HSTRING> : MapChangedEventHandler_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.MapChangedEventHandler`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef MapChangedEventHandler<HSTRING, HSTRING> __FMapChangedEventHandler_2_HSTRING_HSTRING_t;
#define __FMapChangedEventHandler_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FMapChangedEventHandler_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FMapChangedEventHandler_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIObservableMap_2_HSTRING_HSTRING_USE
#define DEF___FIObservableMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1e036276-2f60-55f6-b7f3-f86079e6900b"))
IObservableMap<HSTRING, HSTRING> : IObservableMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IObservableMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IObservableMap<HSTRING, HSTRING> __FIObservableMap_2_HSTRING_HSTRING_t;
#define __FIObservableMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIObservableMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIObservableMap_2_HSTRING_HSTRING_USE */



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

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("66c9da16-d345-5bdb-b953-d86dd9ea8409"))
IVectorView<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*, ABI::Windows::ApplicationModel::Resources::Core::INamedResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Resources.Core.NamedResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Resources::Core::NamedResource*> __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_t;
#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e28e92f0-9ffb-5ea7-9fc9-a73bda471886"))
IVectorView<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*, ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Resources.Core.ResourceCandidate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidate*> __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_t;
#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f2656ef5-fc27-5c24-a8c2-1697e0be736f"))
IVectorView<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*, ABI::Windows::ApplicationModel::Resources::Core::IResourceMap*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Resources.Core.ResourceMap>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Resources::Core::ResourceMap*> __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_t;
#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b82c58dc-1cc0-53f0-b0f4-66ef39a81cd6"))
IVectorView<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*, ABI::Windows::ApplicationModel::Resources::Core::IResourceQualifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Resources.Core.ResourceQualifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifier*> __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_t;
#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                typedef enum CollectionChange : int CollectionChange;
            } /* Collections */
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

namespace ABI {
    namespace Windows {
        namespace UI {
            class UIContext;
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IUIContext;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIUIContext ABI::Windows::UI::IUIContext

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    typedef enum ResourceCandidateKind : int ResourceCandidateKind;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    typedef enum ResourceQualifierPersistence : int ResourceQualifierPersistence;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    typedef struct ResourceLayoutInfo ResourceLayoutInfo;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    class ResourceContext;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    class ResourceManager;
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Resources.Core.ResourceCandidateKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    enum ResourceCandidateKind : int
                    {
                        ResourceCandidateKind_String = 0,
                        ResourceCandidateKind_File = 1,
                        ResourceCandidateKind_EmbeddedData = 2,
                    };
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.ApplicationModel.Resources.Core.ResourceQualifierPersistence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    enum ResourceQualifierPersistence : int
                    {
                        ResourceQualifierPersistence_None = 0,
                        ResourceQualifierPersistence_LocalMachine = 1,
                    };
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Resources.Core.ResourceLayoutInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    struct ResourceLayoutInfo
                    {
                        UINT32 MajorVersion;
                        UINT32 MinorVersion;
                        UINT32 ResourceSubtreeCount;
                        UINT32 NamedResourceCount;
                        INT32 Checksum;
                    };
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.INamedResource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.NamedResource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_INamedResource[] = L"Windows.ApplicationModel.Resources.Core.INamedResource";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("1c98c219-0b13-4240-89a5-d495dc189a00")
                    INamedResource : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Uri(
                            ABI::Windows::Foundation::IUriRuntimeClass** uri
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Candidates(
                            __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Resolve(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResolveForContext(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext* resourceContext,
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResolveAll(
                            __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResolveAllForContext(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext* resourceContext,
                            __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate** instances
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INamedResource = __uuidof(INamedResource);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceCandidate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceCandidate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceCandidate[] = L"Windows.ApplicationModel.Resources.Core.IResourceCandidate";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("af5207d9-c433-4764-b3fd-8fa6bfbcbadc")
                    IResourceCandidate : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Qualifiers(
                            __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsMatch(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsMatchAsDefault(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDefault(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ValueAsString(
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetValueAsFileAsync(
                            __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetQualifierValue(
                            HSTRING qualifierName,
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceCandidate = __uuidof(IResourceCandidate);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceCandidate2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceCandidate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceCandidate2[] = L"Windows.ApplicationModel.Resources.Core.IResourceCandidate2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("69e5b468-f6fc-4013-aaa2-d53f1757d3b5")
                    IResourceCandidate2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetValueAsStreamAsync(
                            __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceCandidate2 = __uuidof(IResourceCandidate2);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceCandidate3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceCandidate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceCandidate3[] = L"Windows.ApplicationModel.Resources.Core.IResourceCandidate3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("08ae97f8-517a-4674-958c-4a3c7cd2cc6b")
                    IResourceCandidate3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::ApplicationModel::Resources::Core::ResourceCandidateKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceCandidate3 = __uuidof(IResourceCandidate3);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContext[] = L"Windows.ApplicationModel.Resources.Core.IResourceContext";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("2fa22f4b-707e-4b27-ad0d-d0d8cd468fd2")
                    IResourceContext : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_QualifierValues(
                            __FIObservableMap_2_HSTRING_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Reset(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResetQualifierValues(
                            __FIIterable_1_HSTRING* qualifierNames
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OverrideToMatch(
                            __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Clone(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext** clone
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Languages(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Languages(
                            __FIVectorView_1_HSTRING* languages
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceContext = __uuidof(IResourceContext);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContextStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContextStatics[] = L"Windows.ApplicationModel.Resources.Core.IResourceContextStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("98be9d6c-6338-4b31-99df-b2b442f17149")
                    IResourceContextStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateMatchingContext(
                            __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* result,
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceContextStatics = __uuidof(IResourceContextStatics);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContextStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContextStatics2[] = L"Windows.ApplicationModel.Resources.Core.IResourceContextStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("41f752ef-12af-41b9-ab36-b1eb4b512460")
                    IResourceContextStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetGlobalQualifierValue(
                            HSTRING key,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResetGlobalQualifierValues(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResetGlobalQualifierValuesForSpecifiedQualifiers(
                            __FIIterable_1_HSTRING* qualifierNames
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetForViewIndependentUse(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext** loader
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceContextStatics2 = __uuidof(IResourceContextStatics2);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContextStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContextStatics3[] = L"Windows.ApplicationModel.Resources.Core.IResourceContextStatics3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("20cf492c-af0f-450b-9da6-106dd0c29a39")
                    IResourceContextStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetGlobalQualifierValueWithPersistence(
                            HSTRING key,
                            HSTRING value,
                            ABI::Windows::ApplicationModel::Resources::Core::ResourceQualifierPersistence persistence
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceContextStatics3 = __uuidof(IResourceContextStatics3);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContextStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContextStatics4[] = L"Windows.ApplicationModel.Resources.Core.IResourceContextStatics4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("22eb9ccd-fb31-4bfa-b86b-df9d9d7bdc39")
                    IResourceContextStatics4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForUIContext(
                            ABI::Windows::UI::IUIContext* context,
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceContextStatics4 = __uuidof(IResourceContextStatics4);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceManager[] = L"Windows.ApplicationModel.Resources.Core.IResourceManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("f744d97b-9988-44fb-abd6-5378844cfa8b")
                    IResourceManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MainResourceMap(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceMap** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AllResourceMaps(
                            __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap** maps
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DefaultContext(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadPriFiles(
                            __FIIterable_1_Windows__CStorage__CIStorageFile* files
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnloadPriFiles(
                            __FIIterable_1_Windows__CStorage__CIStorageFile* files
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceManager = __uuidof(IResourceManager);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceManager2[] = L"Windows.ApplicationModel.Resources.Core.IResourceManager2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("9d66fe6c-a4d7-4c23-9e85-675f304c252d")
                    IResourceManager2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetAllNamedResourcesForPackage(
                            HSTRING packageName,
                            ABI::Windows::ApplicationModel::Resources::Core::ResourceLayoutInfo resourceLayoutInfo,
                            __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource** table
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAllSubtreesForPackage(
                            HSTRING packageName,
                            ABI::Windows::ApplicationModel::Resources::Core::ResourceLayoutInfo resourceLayoutInfo,
                            __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap** table
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceManager2 = __uuidof(IResourceManager2);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceManagerStatics[] = L"Windows.ApplicationModel.Resources.Core.IResourceManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("1cc0fdfc-69ee-4e43-9901-47f12687baf7")
                    IResourceManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Current(
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceManager** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsResourceReference(
                            HSTRING resourceReference,
                            boolean* isReference
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceManagerStatics = __uuidof(IResourceManagerStatics);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceMap
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceMap[] = L"Windows.ApplicationModel.Resources.Core.IResourceMap";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("72284824-db8c-42f8-b08c-53ff357dad82")
                    IResourceMap : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Uri(
                            ABI::Windows::Foundation::IUriRuntimeClass** uri
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetValue(
                            HSTRING resource,
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetValueForContext(
                            HSTRING resource,
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceContext* context,
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceCandidate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSubtree(
                            HSTRING reference,
                            ABI::Windows::ApplicationModel::Resources::Core::IResourceMap** map
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceMap = __uuidof(IResourceMap);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceQualifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceQualifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceQualifier[] = L"Windows.ApplicationModel.Resources.Core.IResourceQualifier";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Resources {
                namespace Core {
                    MIDL_INTERFACE("785da5b2-4afd-4376-a888-c5f9a6b7a05c")
                    IResourceQualifier : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_QualifierName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_QualifierValue(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDefault(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsMatch(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Score(
                            DOUBLE* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IResourceQualifier = __uuidof(IResourceQualifier);
                } /* Core */
            } /* Resources */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.NamedResource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.INamedResource ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_NamedResource_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_NamedResource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_NamedResource[] = L"Windows.ApplicationModel.Resources.Core.NamedResource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceCandidate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceCandidate ** Default Interface **
 *    Windows.ApplicationModel.Resources.Core.IResourceCandidate2
 *    Windows.ApplicationModel.Resources.Core.IResourceCandidate3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceCandidate_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceCandidate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceCandidate[] = L"Windows.ApplicationModel.Resources.Core.ResourceCandidate";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Resources.Core.ResourceCandidate> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Resources.Core.ResourceCandidate>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceCandidateVectorView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceCandidateVectorView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceCandidateVectorView[] = L"Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceContextStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceContextStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceContextStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceContextStatics4 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceContext_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceContext[] = L"Windows.ApplicationModel.Resources.Core.ResourceContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<String> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<String>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceContextLanguagesVectorView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceContextLanguagesVectorView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceContextLanguagesVectorView[] = L"Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceManager ** Default Interface **
 *    Windows.ApplicationModel.Resources.Core.IResourceManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceManager[] = L"Windows.ApplicationModel.Resources.Core.ResourceManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceMap ** Default Interface **
 *    Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMap_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceMap[] = L"Windows.ApplicationModel.Resources.Core.ResourceMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceMapIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>> ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapIterator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceMapIterator[] = L"Windows.ApplicationModel.Resources.Core.ResourceMapIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceMapMapView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapMapView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapMapView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceMapMapView[] = L"Windows.ApplicationModel.Resources.Core.ResourceMapMapView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap>> ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapMapViewIterator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapMapViewIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceMapMapViewIterator[] = L"Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceQualifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceQualifier ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifier_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceQualifier[] = L"Windows.ApplicationModel.Resources.Core.ResourceQualifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMapView`2<String, String> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierMapView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierMapView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceQualifierMapView[] = L"Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IObservableMap`2<String, String> ** Default Interface **
 *    Windows.Foundation.Collections.IMap`2<String, String>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierObservableMap_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierObservableMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceQualifierObservableMap[] = L"Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Resources.Core.ResourceQualifier> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Resources.Core.ResourceQualifier>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierVectorView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierVectorView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceQualifierVectorView[] = L"Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2 __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3 __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2 __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3 __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4 __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2 __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier;

#endif // ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
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
#if !defined(____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource;

typedef struct __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource;

typedef struct __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CNamedResource** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate;

typedef struct __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate;

typedef struct __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap;

typedef struct __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap;

typedef struct __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceMap** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier;

typedef struct __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier;

typedef struct __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        __FIIterator_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CIStorageFile_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CIStorageFile_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CIStorageFile __FIIterator_1_Windows__CStorage__CIStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CIStorageFile;

typedef struct __FIIterator_1_Windows__CStorage__CIStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CIStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CIStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CIStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CIStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CIStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CIStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CIStorageFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CIStorageFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CIStorageFile* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFile** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CIStorageFileVtbl;

interface __FIIterator_1_Windows__CStorage__CIStorageFile
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CIStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CIStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageFile_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CIStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CIStorageFile_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CIStorageFile_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CIStorageFile __FIIterable_1_Windows__CStorage__CIStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CIStorageFile;

typedef struct __FIIterable_1_Windows__CStorage__CIStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CIStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CIStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CIStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CIStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CIStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CIStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CIStorageFile* This,
        __FIIterator_1_Windows__CStorage__CIStorageFile** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CIStorageFileVtbl;

interface __FIIterable_1_Windows__CStorage__CIStorageFile
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CIStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CIStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CIStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CIStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CIStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CIStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CIStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CIStorageFile_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CIStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapChangedEventArgs_1_HSTRING __FIMapChangedEventArgs_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapChangedEventArgs_1_HSTRING;

typedef struct __FIMapChangedEventArgs_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapChangedEventArgs_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapChangedEventArgs_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapChangedEventArgs_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapChangedEventArgs_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapChangedEventArgs_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapChangedEventArgs_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CollectionChange)(__FIMapChangedEventArgs_1_HSTRING* This,
        enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange* result);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIMapChangedEventArgs_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIMapChangedEventArgs_1_HSTRINGVtbl;

interface __FIMapChangedEventArgs_1_HSTRING
{
    CONST_VTBL struct __FIMapChangedEventArgs_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapChangedEventArgs_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapChangedEventArgs_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapChangedEventArgs_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapChangedEventArgs_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapChangedEventArgs_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapChangedEventArgs_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapChangedEventArgs_1_HSTRING_get_CollectionChange(This, result) \
    ((This)->lpVtbl->get_CollectionChange(This, result))

#define __FIMapChangedEventArgs_1_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#endif /* COBJMACROS */

#endif // ____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__

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

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

typedef interface __FIObservableMap_2_HSTRING_HSTRING __FIObservableMap_2_HSTRING_HSTRING;

#if !defined(____FMapChangedEventHandler_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FMapChangedEventHandler_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FMapChangedEventHandler_2_HSTRING_HSTRING __FMapChangedEventHandler_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FMapChangedEventHandler_2_HSTRING_HSTRING;

typedef struct __FMapChangedEventHandler_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FMapChangedEventHandler_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FMapChangedEventHandler_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FMapChangedEventHandler_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FMapChangedEventHandler_2_HSTRING_HSTRING* This,
        __FIObservableMap_2_HSTRING_HSTRING* sender,
        __FIMapChangedEventArgs_1_HSTRING* event);

    END_INTERFACE
} __FMapChangedEventHandler_2_HSTRING_HSTRINGVtbl;

interface __FMapChangedEventHandler_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FMapChangedEventHandler_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FMapChangedEventHandler_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FMapChangedEventHandler_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FMapChangedEventHandler_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FMapChangedEventHandler_2_HSTRING_HSTRING_Invoke(This, sender, event) \
    ((This)->lpVtbl->Invoke(This, sender, event))

#endif /* COBJMACROS */

#endif // ____FMapChangedEventHandler_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIObservableMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIObservableMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIObservableMap_2_HSTRING_HSTRING __FIObservableMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIObservableMap_2_HSTRING_HSTRING;

typedef struct __FIObservableMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIObservableMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIObservableMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIObservableMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIObservableMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIObservableMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIObservableMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MapChanged)(__FIObservableMap_2_HSTRING_HSTRING* This,
        __FMapChangedEventHandler_2_HSTRING_HSTRING* vhnd,
        EventRegistrationToken* result);
    HRESULT (STDMETHODCALLTYPE* remove_MapChanged)(__FIObservableMap_2_HSTRING_HSTRING* This,
        EventRegistrationToken token);

    END_INTERFACE
} __FIObservableMap_2_HSTRING_HSTRINGVtbl;

interface __FIObservableMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIObservableMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIObservableMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIObservableMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIObservableMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIObservableMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIObservableMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIObservableMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIObservableMap_2_HSTRING_HSTRING_add_MapChanged(This, vhnd, result) \
    ((This)->lpVtbl->add_MapChanged(This, vhnd, result))

#define __FIObservableMap_2_HSTRING_HSTRING_remove_MapChanged(This, token) \
    ((This)->lpVtbl->remove_MapChanged(This, token))

#endif /* COBJMACROS */

#endif // ____FIObservableMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

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
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContext __x_ABI_CWindows_CUI_CIUIContext;

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceCandidateKind __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceCandidateKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceQualifierPersistence __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceQualifierPersistence;

typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceLayoutInfo __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceLayoutInfo;

/*
 *
 * Struct Windows.ApplicationModel.Resources.Core.ResourceCandidateKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceCandidateKind
{
    ResourceCandidateKind_String = 0,
    ResourceCandidateKind_File = 1,
    ResourceCandidateKind_EmbeddedData = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.ApplicationModel.Resources.Core.ResourceQualifierPersistence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceQualifierPersistence
{
    ResourceQualifierPersistence_None = 0,
    ResourceQualifierPersistence_LocalMachine = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Resources.Core.ResourceLayoutInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceLayoutInfo
{
    UINT32 MajorVersion;
    UINT32 MinorVersion;
    UINT32 ResourceSubtreeCount;
    UINT32 NamedResourceCount;
    INT32 Checksum;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.INamedResource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.NamedResource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_INamedResource[] = L"Windows.ApplicationModel.Resources.Core.INamedResource";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** uri);
    HRESULT (STDMETHODCALLTYPE* get_Candidates)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate** value);
    HRESULT (STDMETHODCALLTYPE* Resolve)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate** result);
    HRESULT (STDMETHODCALLTYPE* ResolveForContext)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* resourceContext,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate** result);
    HRESULT (STDMETHODCALLTYPE* ResolveAll)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate** result);
    HRESULT (STDMETHODCALLTYPE* ResolveAllForContext)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* resourceContext,
        __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceCandidate** instances);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResourceVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_get_Uri(This, uri) \
    ((This)->lpVtbl->get_Uri(This, uri))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_get_Candidates(This, value) \
    ((This)->lpVtbl->get_Candidates(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_Resolve(This, result) \
    ((This)->lpVtbl->Resolve(This, result))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_ResolveForContext(This, resourceContext, result) \
    ((This)->lpVtbl->ResolveForContext(This, resourceContext, result))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_ResolveAll(This, result) \
    ((This)->lpVtbl->ResolveAll(This, result))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_ResolveAllForContext(This, resourceContext, instances) \
    ((This)->lpVtbl->ResolveAllForContext(This, resourceContext, instances))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CINamedResource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceCandidate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceCandidate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceCandidate[] = L"Windows.ApplicationModel.Resources.Core.IResourceCandidate";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Qualifiers)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier** value);
    HRESULT (STDMETHODCALLTYPE* get_IsMatch)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMatchAsDefault)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDefault)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ValueAsString)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetValueAsFileAsync)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* GetQualifierValue)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate* This,
        HSTRING qualifierName,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidateVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_get_Qualifiers(This, value) \
    ((This)->lpVtbl->get_Qualifiers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_get_IsMatch(This, value) \
    ((This)->lpVtbl->get_IsMatch(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_get_IsMatchAsDefault(This, value) \
    ((This)->lpVtbl->get_IsMatchAsDefault(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_get_IsDefault(This, value) \
    ((This)->lpVtbl->get_IsDefault(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_get_ValueAsString(This, result) \
    ((This)->lpVtbl->get_ValueAsString(This, result))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_GetValueAsFileAsync(This, operation) \
    ((This)->lpVtbl->GetValueAsFileAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_GetQualifierValue(This, qualifierName, value) \
    ((This)->lpVtbl->GetQualifierValue(This, qualifierName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceCandidate2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceCandidate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceCandidate2[] = L"Windows.ApplicationModel.Resources.Core.IResourceCandidate2";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetValueAsStreamAsync)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_GetValueAsStreamAsync(This, operation) \
    ((This)->lpVtbl->GetValueAsStreamAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceCandidate3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceCandidate
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceCandidate3[] = L"Windows.ApplicationModel.Resources.Core.IResourceCandidate3";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3* This,
        enum __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceCandidateKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContext[] = L"Windows.ApplicationModel.Resources.Core.IResourceContext";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_QualifierValues)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        __FIObservableMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* Reset)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This);
    HRESULT (STDMETHODCALLTYPE* ResetQualifierValues)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        __FIIterable_1_HSTRING* qualifierNames);
    HRESULT (STDMETHODCALLTYPE* OverrideToMatch)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* result);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext** clone);
    HRESULT (STDMETHODCALLTYPE* get_Languages)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* put_Languages)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* This,
        __FIVectorView_1_HSTRING* languages);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_get_QualifierValues(This, value) \
    ((This)->lpVtbl->get_QualifierValues(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_Reset(This) \
    ((This)->lpVtbl->Reset(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_ResetQualifierValues(This, qualifierNames) \
    ((This)->lpVtbl->ResetQualifierValues(This, qualifierNames))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_OverrideToMatch(This, result) \
    ((This)->lpVtbl->OverrideToMatch(This, result))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_Clone(This, clone) \
    ((This)->lpVtbl->Clone(This, clone))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_get_Languages(This, value) \
    ((This)->lpVtbl->get_Languages(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_put_Languages(This, languages) \
    ((This)->lpVtbl->put_Languages(This, languages))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContextStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContextStatics[] = L"Windows.ApplicationModel.Resources.Core.IResourceContextStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateMatchingContext)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics* This,
        __FIIterable_1_Windows__CApplicationModel__CResources__CCore__CResourceQualifier* result,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_CreateMatchingContext(This, result, value) \
    ((This)->lpVtbl->CreateMatchingContext(This, result, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContextStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContextStatics2[] = L"Windows.ApplicationModel.Resources.Core.IResourceContextStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext** value);
    HRESULT (STDMETHODCALLTYPE* SetGlobalQualifierValue)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This,
        HSTRING key,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* ResetGlobalQualifierValues)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This);
    HRESULT (STDMETHODCALLTYPE* ResetGlobalQualifierValuesForSpecifiedQualifiers)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This,
        __FIIterable_1_HSTRING* qualifierNames);
    HRESULT (STDMETHODCALLTYPE* GetForViewIndependentUse)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext** loader);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_GetForCurrentView(This, value) \
    ((This)->lpVtbl->GetForCurrentView(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_SetGlobalQualifierValue(This, key, value) \
    ((This)->lpVtbl->SetGlobalQualifierValue(This, key, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_ResetGlobalQualifierValues(This) \
    ((This)->lpVtbl->ResetGlobalQualifierValues(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_ResetGlobalQualifierValuesForSpecifiedQualifiers(This, qualifierNames) \
    ((This)->lpVtbl->ResetGlobalQualifierValuesForSpecifiedQualifiers(This, qualifierNames))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_GetForViewIndependentUse(This, loader) \
    ((This)->lpVtbl->GetForViewIndependentUse(This, loader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContextStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContextStatics3[] = L"Windows.ApplicationModel.Resources.Core.IResourceContextStatics3";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetGlobalQualifierValueWithPersistence)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3* This,
        HSTRING key,
        HSTRING value,
        enum __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceQualifierPersistence persistence);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_SetGlobalQualifierValueWithPersistence(This, key, value, persistence) \
    ((This)->lpVtbl->SetGlobalQualifierValueWithPersistence(This, key, value, persistence))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceContextStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceContextStatics4[] = L"Windows.ApplicationModel.Resources.Core.IResourceContextStatics4";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUIContext)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4* This,
        __x_ABI_CWindows_CUI_CIUIContext* context,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_GetForUIContext(This, context, value) \
    ((This)->lpVtbl->GetForUIContext(This, context, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContextStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceManager[] = L"Windows.ApplicationModel.Resources.Core.IResourceManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MainResourceMap)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap** value);
    HRESULT (STDMETHODCALLTYPE* get_AllResourceMaps)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CResources__CCore__CResourceMap** maps);
    HRESULT (STDMETHODCALLTYPE* get_DefaultContext)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext** value);
    HRESULT (STDMETHODCALLTYPE* LoadPriFiles)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        __FIIterable_1_Windows__CStorage__CIStorageFile* files);
    HRESULT (STDMETHODCALLTYPE* UnloadPriFiles)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager* This,
        __FIIterable_1_Windows__CStorage__CIStorageFile* files);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_get_MainResourceMap(This, value) \
    ((This)->lpVtbl->get_MainResourceMap(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_get_AllResourceMaps(This, maps) \
    ((This)->lpVtbl->get_AllResourceMaps(This, maps))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_get_DefaultContext(This, value) \
    ((This)->lpVtbl->get_DefaultContext(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_LoadPriFiles(This, files) \
    ((This)->lpVtbl->LoadPriFiles(This, files))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_UnloadPriFiles(This, files) \
    ((This)->lpVtbl->UnloadPriFiles(This, files))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceManager2[] = L"Windows.ApplicationModel.Resources.Core.IResourceManager2";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAllNamedResourcesForPackage)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2* This,
        HSTRING packageName,
        struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceLayoutInfo resourceLayoutInfo,
        __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CNamedResource** table);
    HRESULT (STDMETHODCALLTYPE* GetAllSubtreesForPackage)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2* This,
        HSTRING packageName,
        struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CResourceLayoutInfo resourceLayoutInfo,
        __FIVectorView_1_Windows__CApplicationModel__CResources__CCore__CResourceMap** table);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_GetAllNamedResourcesForPackage(This, packageName, resourceLayoutInfo, table) \
    ((This)->lpVtbl->GetAllNamedResourcesForPackage(This, packageName, resourceLayoutInfo, table))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_GetAllSubtreesForPackage(This, packageName, resourceLayoutInfo, table) \
    ((This)->lpVtbl->GetAllSubtreesForPackage(This, packageName, resourceLayoutInfo, table))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceManagerStatics[] = L"Windows.ApplicationModel.Resources.Core.IResourceManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManager** value);
    HRESULT (STDMETHODCALLTYPE* IsResourceReference)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics* This,
        HSTRING resourceReference,
        boolean* isReference);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_IsResourceReference(This, resourceReference, isReference) \
    ((This)->lpVtbl->IsResourceReference(This, resourceReference, isReference))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceMap
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceMap[] = L"Windows.ApplicationModel.Resources.Core.IResourceMap";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** uri);
    HRESULT (STDMETHODCALLTYPE* GetValue)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This,
        HSTRING resource,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate** value);
    HRESULT (STDMETHODCALLTYPE* GetValueForContext)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This,
        HSTRING resource,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceContext* context,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceCandidate** value);
    HRESULT (STDMETHODCALLTYPE* GetSubtree)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap* This,
        HSTRING reference,
        __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap** map);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMapVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_get_Uri(This, uri) \
    ((This)->lpVtbl->get_Uri(This, uri))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_GetValue(This, resource, value) \
    ((This)->lpVtbl->GetValue(This, resource, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_GetValueForContext(This, resource, context, value) \
    ((This)->lpVtbl->GetValueForContext(This, resource, context, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_GetSubtree(This, reference, map) \
    ((This)->lpVtbl->GetSubtree(This, reference, map))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceMap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Resources.Core.IResourceQualifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Resources.Core.ResourceQualifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Resources_Core_IResourceQualifier[] = L"Windows.ApplicationModel.Resources.Core.IResourceQualifier";
typedef struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_QualifierName)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_QualifierValue)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDefault)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMatch)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Score)(__x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifierVtbl;

interface __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_get_QualifierName(This, value) \
    ((This)->lpVtbl->get_QualifierName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_get_QualifierValue(This, value) \
    ((This)->lpVtbl->get_QualifierValue(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_get_IsDefault(This, value) \
    ((This)->lpVtbl->get_IsDefault(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_get_IsMatch(This, value) \
    ((This)->lpVtbl->get_IsMatch(This, value))

#define __x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_get_Score(This, value) \
    ((This)->lpVtbl->get_Score(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CResources_CCore_CIResourceQualifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.NamedResource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.INamedResource ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_NamedResource_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_NamedResource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_NamedResource[] = L"Windows.ApplicationModel.Resources.Core.NamedResource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceCandidate
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceCandidate ** Default Interface **
 *    Windows.ApplicationModel.Resources.Core.IResourceCandidate2
 *    Windows.ApplicationModel.Resources.Core.IResourceCandidate3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceCandidate_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceCandidate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceCandidate[] = L"Windows.ApplicationModel.Resources.Core.ResourceCandidate";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Resources.Core.ResourceCandidate> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Resources.Core.ResourceCandidate>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceCandidateVectorView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceCandidateVectorView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceCandidateVectorView[] = L"Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceContextStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceContextStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceContextStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceContextStatics4 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceContext_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceContext[] = L"Windows.ApplicationModel.Resources.Core.ResourceContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<String> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<String>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceContextLanguagesVectorView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceContextLanguagesVectorView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceContextLanguagesVectorView[] = L"Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Resources.Core.IResourceManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceManager ** Default Interface **
 *    Windows.ApplicationModel.Resources.Core.IResourceManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceManager[] = L"Windows.ApplicationModel.Resources.Core.ResourceManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceMap ** Default Interface **
 *    Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMap_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceMap[] = L"Windows.ApplicationModel.Resources.Core.ResourceMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceMapIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.NamedResource>> ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapIterator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceMapIterator[] = L"Windows.ApplicationModel.Resources.Core.ResourceMapIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceMapMapView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapMapView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapMapView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceMapMapView[] = L"Windows.ApplicationModel.Resources.Core.ResourceMapMapView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Resources.Core.ResourceMap>> ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapMapViewIterator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceMapMapViewIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceMapMapViewIterator[] = L"Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceQualifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Resources.Core.IResourceQualifier ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifier_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceQualifier[] = L"Windows.ApplicationModel.Resources.Core.ResourceQualifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMapView`2<String, String> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierMapView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierMapView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceQualifierMapView[] = L"Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IObservableMap`2<String, String> ** Default Interface **
 *    Windows.Foundation.Collections.IMap`2<String, String>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierObservableMap_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierObservableMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceQualifierObservableMap[] = L"Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Resources.Core.ResourceQualifier> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Resources.Core.ResourceQualifier>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierVectorView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Resources_Core_ResourceQualifierVectorView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Resources_Core_ResourceQualifierVectorView[] = L"Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView";
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
#endif // __windows2Eapplicationmodel2Eresources2Ecore_p_h__

#endif // __windows2Eapplicationmodel2Eresources2Ecore_h__
