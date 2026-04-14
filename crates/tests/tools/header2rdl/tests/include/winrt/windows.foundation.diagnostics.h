
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
#ifndef __windows2Efoundation2Ediagnostics_h__
#define __windows2Efoundation2Ediagnostics_h__
#ifndef __windows2Efoundation2Ediagnostics_p_h__
#define __windows2Efoundation2Ediagnostics_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface IAsyncCausalityTracerStatics;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics ABI::Windows::Foundation::Diagnostics::IAsyncCausalityTracerStatics

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface IErrorDetails;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails ABI::Windows::Foundation::Diagnostics::IErrorDetails

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface IErrorDetailsStatics;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics ABI::Windows::Foundation::Diagnostics::IErrorDetailsStatics

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface IErrorReportingSettings;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings ABI::Windows::Foundation::Diagnostics::IErrorReportingSettings

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface IFileLoggingSession;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession ABI::Windows::Foundation::Diagnostics::IFileLoggingSession

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface IFileLoggingSessionFactory;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory ABI::Windows::Foundation::Diagnostics::IFileLoggingSessionFactory

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILogFileGeneratedEventArgs;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs ABI::Windows::Foundation::Diagnostics::ILogFileGeneratedEventArgs

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingActivity;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity ABI::Windows::Foundation::Diagnostics::ILoggingActivity

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingActivity2;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2 ABI::Windows::Foundation::Diagnostics::ILoggingActivity2

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingActivityFactory;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory ABI::Windows::Foundation::Diagnostics::ILoggingActivityFactory

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingChannel;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel ABI::Windows::Foundation::Diagnostics::ILoggingChannel

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingChannel2;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2 ABI::Windows::Foundation::Diagnostics::ILoggingChannel2

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingChannelFactory;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory ABI::Windows::Foundation::Diagnostics::ILoggingChannelFactory

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingChannelFactory2;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2 ABI::Windows::Foundation::Diagnostics::ILoggingChannelFactory2

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingChannelOptions;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions ABI::Windows::Foundation::Diagnostics::ILoggingChannelOptions

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingChannelOptionsFactory;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory ABI::Windows::Foundation::Diagnostics::ILoggingChannelOptionsFactory

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingFields;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields ABI::Windows::Foundation::Diagnostics::ILoggingFields

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingOptions;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions ABI::Windows::Foundation::Diagnostics::ILoggingOptions

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingOptionsFactory;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory ABI::Windows::Foundation::Diagnostics::ILoggingOptionsFactory

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingSession;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession ABI::Windows::Foundation::Diagnostics::ILoggingSession

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingSessionFactory;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory ABI::Windows::Foundation::Diagnostics::ILoggingSessionFactory

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ILoggingTarget;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget ABI::Windows::Foundation::Diagnostics::ILoggingTarget

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                interface ITracingStatusChangedEventArgs;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs ABI::Windows::Foundation::Diagnostics::ITracingStatusChangedEventArgs

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class ErrorDetails;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_USE
#define DEF___FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9b05106d-77e0-5c24-82b0-9b2dc8f79671"))
IAsyncOperation<ABI::Windows::Foundation::Diagnostics::ErrorDetails*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Diagnostics::ErrorDetails*, ABI::Windows::Foundation::Diagnostics::IErrorDetails*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Diagnostics.ErrorDetails>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Foundation::Diagnostics::ErrorDetails*> __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_t;
#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a6997f9d-7195-5972-8ecd-1c73aa5cb312"))
IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Diagnostics::ErrorDetails*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Diagnostics::ErrorDetails*, ABI::Windows::Foundation::Diagnostics::IErrorDetails*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Diagnostics.ErrorDetails>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Diagnostics::ErrorDetails*> __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class TracingStatusChangedEventArgs;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2bf27008-2eb4-5675-b1cd-e9906cc5ce64"))
IEventHandler<ABI::Windows::Foundation::Diagnostics::TracingStatusChangedEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Diagnostics::TracingStatusChangedEventArgs*, ABI::Windows::Foundation::Diagnostics::ITracingStatusChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Foundation::Diagnostics::TracingStatusChangedEventArgs*> __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_t;
#define __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class LogFileGeneratedEventArgs;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0c6563b0-9d8b-5b60-994b-dee1174d1efb"))
ITypedEventHandler<ABI::Windows::Foundation::Diagnostics::IFileLoggingSession*, ABI::Windows::Foundation::Diagnostics::LogFileGeneratedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Diagnostics::IFileLoggingSession*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Diagnostics::LogFileGeneratedEventArgs*, ABI::Windows::Foundation::Diagnostics::ILogFileGeneratedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Foundation.Diagnostics.IFileLoggingSession, Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Foundation::Diagnostics::IFileLoggingSession*, ABI::Windows::Foundation::Diagnostics::LogFileGeneratedEventArgs*> __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("52c9c2a1-54a3-5ef9-9aff-014e7c454655"))
ITypedEventHandler<ABI::Windows::Foundation::Diagnostics::ILoggingChannel*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Diagnostics::ILoggingChannel*, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Foundation.Diagnostics.ILoggingChannel, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Foundation::Diagnostics::ILoggingChannel*, IInspectable*> __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                typedef enum CausalityRelation : int CausalityRelation;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                typedef enum CausalitySource : int CausalitySource;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                typedef enum CausalitySynchronousWork : int CausalitySynchronousWork;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                typedef enum CausalityTraceLevel : int CausalityTraceLevel;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                typedef enum ErrorOptions : unsigned int ErrorOptions;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                typedef enum LoggingFieldFormat : int LoggingFieldFormat;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                typedef enum LoggingLevel : int LoggingLevel;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                typedef enum LoggingOpcode : int LoggingOpcode;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class FileLoggingSession;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class LoggingActivity;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class LoggingChannel;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class LoggingChannelOptions;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class LoggingFields;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class LoggingOptions;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                class LoggingSession;
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Foundation.Diagnostics.CausalityRelation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                enum CausalityRelation : int
                {
                    CausalityRelation_AssignDelegate = 0,
                    CausalityRelation_Join = 1,
                    CausalityRelation_Choice = 2,
                    CausalityRelation_Cancel = 3,
                    CausalityRelation_Error = 4,
                };
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.CausalitySource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                enum CausalitySource : int
                {
                    CausalitySource_Application = 0,
                    CausalitySource_Library = 1,
                    CausalitySource_System = 2,
                };
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.CausalitySynchronousWork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                enum CausalitySynchronousWork : int
                {
                    CausalitySynchronousWork_CompletionNotification = 0,
                    CausalitySynchronousWork_ProgressNotification = 1,
                    CausalitySynchronousWork_Execution = 2,
                };
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.CausalityTraceLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                enum CausalityTraceLevel : int
                {
                    CausalityTraceLevel_Required = 0,
                    CausalityTraceLevel_Important = 1,
                    CausalityTraceLevel_Verbose = 2,
                };
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.ErrorOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                enum ErrorOptions : unsigned int
                {
                    ErrorOptions_None = 0,
                    ErrorOptions_SuppressExceptions = 0x1,
                    ErrorOptions_ForceExceptions = 0x2,
                    ErrorOptions_UseSetErrorInfo = 0x4,
                    ErrorOptions_SuppressSetErrorInfo = 0x8,
                };

                DEFINE_ENUM_FLAG_OPERATORS(ErrorOptions)
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.LoggingFieldFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                enum LoggingFieldFormat : int
                {
                    LoggingFieldFormat_Default = 0,
                    LoggingFieldFormat_Hidden = 1,
                    LoggingFieldFormat_String = 2,
                    LoggingFieldFormat_Boolean = 3,
                    LoggingFieldFormat_Hexadecimal = 4,
                    LoggingFieldFormat_ProcessId = 5,
                    LoggingFieldFormat_ThreadId = 6,
                    LoggingFieldFormat_Port = 7,
                    LoggingFieldFormat_Ipv4Address = 8,
                    LoggingFieldFormat_Ipv6Address = 9,
                    LoggingFieldFormat_SocketAddress = 10,
                    LoggingFieldFormat_Xml = 11,
                    LoggingFieldFormat_Json = 12,
                    LoggingFieldFormat_Win32Error = 13,
                    LoggingFieldFormat_NTStatus = 14,
                    LoggingFieldFormat_HResult = 15,
                    LoggingFieldFormat_FileTime = 16,
                    LoggingFieldFormat_Signed = 17,
                    LoggingFieldFormat_Unsigned = 18,
                };
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.LoggingLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                enum LoggingLevel : int
                {
                    LoggingLevel_Verbose = 0,
                    LoggingLevel_Information = 1,
                    LoggingLevel_Warning = 2,
                    LoggingLevel_Error = 3,
                    LoggingLevel_Critical = 4,
                };
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.LoggingOpcode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                enum LoggingOpcode : int
                {
                    LoggingOpcode_Info = 0,
                    LoggingOpcode_Start = 1,
                    LoggingOpcode_Stop = 2,
                    LoggingOpcode_Reply = 6,
                    LoggingOpcode_Resume = 7,
                    LoggingOpcode_Suspend = 8,
                    LoggingOpcode_Send = 9,
                };
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IAsyncCausalityTracerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.AsyncCausalityTracer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IAsyncCausalityTracerStatics[] = L"Windows.Foundation.Diagnostics.IAsyncCausalityTracerStatics";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("50850b26-267e-451b-a890-ab6a370245ee")
                IAsyncCausalityTracerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TraceOperationCreation(
                        ABI::Windows::Foundation::Diagnostics::CausalityTraceLevel traceLevel,
                        ABI::Windows::Foundation::Diagnostics::CausalitySource source,
                        GUID platformId,
                        UINT64 operationId,
                        HSTRING operationName,
                        UINT64 relatedContext
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TraceOperationCompletion(
                        ABI::Windows::Foundation::Diagnostics::CausalityTraceLevel traceLevel,
                        ABI::Windows::Foundation::Diagnostics::CausalitySource source,
                        GUID platformId,
                        UINT64 operationId,
                        AsyncStatus status
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TraceOperationRelation(
                        ABI::Windows::Foundation::Diagnostics::CausalityTraceLevel traceLevel,
                        ABI::Windows::Foundation::Diagnostics::CausalitySource source,
                        GUID platformId,
                        UINT64 operationId,
                        ABI::Windows::Foundation::Diagnostics::CausalityRelation relation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TraceSynchronousWorkStart(
                        ABI::Windows::Foundation::Diagnostics::CausalityTraceLevel traceLevel,
                        ABI::Windows::Foundation::Diagnostics::CausalitySource source,
                        GUID platformId,
                        UINT64 operationId,
                        ABI::Windows::Foundation::Diagnostics::CausalitySynchronousWork work
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TraceSynchronousWorkCompletion(
                        ABI::Windows::Foundation::Diagnostics::CausalityTraceLevel traceLevel,
                        ABI::Windows::Foundation::Diagnostics::CausalitySource source,
                        ABI::Windows::Foundation::Diagnostics::CausalitySynchronousWork work
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TracingStatusChanged(
                        __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TracingStatusChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAsyncCausalityTracerStatics = __uuidof(IAsyncCausalityTracerStatics);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IErrorDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.ErrorDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IErrorDetails[] = L"Windows.Foundation.Diagnostics.IErrorDetails";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("378cbb01-2cc9-428f-8c55-2c990d463e8f")
                IErrorDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LongDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HelpUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IErrorDetails = __uuidof(IErrorDetails);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IErrorDetailsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.ErrorDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IErrorDetailsStatics[] = L"Windows.Foundation.Diagnostics.IErrorDetailsStatics";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("b7703750-0b1d-46c8-aa0e-4b8178e4fce9")
                IErrorDetailsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromHResultAsync(
                        INT32 errorCode,
                        __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IErrorDetailsStatics = __uuidof(IErrorDetailsStatics);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IErrorReportingSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IErrorReportingSettings[] = L"Windows.Foundation.Diagnostics.IErrorReportingSettings";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("16369792-b03e-4ba1-8bb8-d28f4ab4d2c0")
                IErrorReportingSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetErrorOptions(
                        ABI::Windows::Foundation::Diagnostics::ErrorOptions value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetErrorOptions(
                        ABI::Windows::Foundation::Diagnostics::ErrorOptions* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IErrorReportingSettings = __uuidof(IErrorReportingSettings);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IFileLoggingSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IFileLoggingSession[] = L"Windows.Foundation.Diagnostics.IFileLoggingSession";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("24c74216-fed2-404c-895f-1f9699cb02f7")
                IFileLoggingSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddLoggingChannel(
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel* loggingChannel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddLoggingChannelWithLevel(
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel* loggingChannel,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel maxLevel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveLoggingChannel(
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel* loggingChannel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CloseAndSaveToFileAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LogFileGenerated(
                        __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LogFileGenerated(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileLoggingSession = __uuidof(IFileLoggingSession);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IFileLoggingSessionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.FileLoggingSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IFileLoggingSessionFactory[] = L"Windows.Foundation.Diagnostics.IFileLoggingSessionFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("eea08dce-8447-4daa-9133-12eb46f697d4")
                IFileLoggingSessionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING name,
                        ABI::Windows::Foundation::Diagnostics::IFileLoggingSession** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileLoggingSessionFactory = __uuidof(IFileLoggingSessionFactory);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILogFileGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILogFileGeneratedEventArgs[] = L"Windows.Foundation.Diagnostics.ILogFileGeneratedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("269e976f-0d38-4c1a-b53f-b395d881df84")
                ILogFileGeneratedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_File(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILogFileGeneratedEventArgs = __uuidof(ILogFileGeneratedEventArgs);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingActivity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingActivity
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingActivity[] = L"Windows.Foundation.Diagnostics.ILoggingActivity";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("bc032941-b766-4cb5-9848-97ac6ba6d60c")
                ILoggingActivity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingActivity = __uuidof(ILoggingActivity);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingActivity2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingActivity
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Diagnostics.ILoggingActivity
 *     Windows.Foundation.IClosable
 *     Windows.Foundation.Diagnostics.ILoggingTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingActivity2[] = L"Windows.Foundation.Diagnostics.ILoggingActivity2";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("26c29808-6322-456a-af82-80c8642f178b")
                ILoggingActivity2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Channel(
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopActivity(
                        HSTRING stopEventName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopActivityWithFields(
                        HSTRING stopEventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingFields* fields
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopActivityWithFieldsAndOptions(
                        HSTRING stopEventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingFields* fields,
                        ABI::Windows::Foundation::Diagnostics::ILoggingOptions* options
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingActivity2 = __uuidof(ILoggingActivity2);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingActivityFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingActivity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingActivityFactory[] = L"Windows.Foundation.Diagnostics.ILoggingActivityFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("6b33b483-e10a-4c58-97d5-10fb451074fb")
                ILoggingActivityFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateLoggingActivity(
                        HSTRING activityName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel* loggingChannel,
                        ABI::Windows::Foundation::Diagnostics::ILoggingActivity** loggingActivity
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateLoggingActivityWithLevel(
                        HSTRING activityName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel* loggingChannel,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level,
                        ABI::Windows::Foundation::Diagnostics::ILoggingActivity** loggingActivity
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingActivityFactory = __uuidof(ILoggingActivityFactory);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannel[] = L"Windows.Foundation.Diagnostics.ILoggingChannel";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("e9a50343-11d7-4f01-b5ca-cf495278c0a8")
                ILoggingChannel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Level(
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogMessage(
                        HSTRING eventString
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogMessageWithLevel(
                        HSTRING eventString,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogValuePair(
                        HSTRING value1,
                        INT32 value2
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogValuePairWithLevel(
                        HSTRING value1,
                        INT32 value2,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LoggingEnabled(
                        __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LoggingEnabled(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingChannel = __uuidof(ILoggingChannel);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannel2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannel
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Diagnostics.ILoggingChannel
 *     Windows.Foundation.IClosable
 *     Windows.Foundation.Diagnostics.ILoggingTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannel2[] = L"Windows.Foundation.Diagnostics.ILoggingChannel2";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("9f4c3cf3-0bac-45a5-9e33-baf3f3a246a5")
                ILoggingChannel2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingChannel2 = __uuidof(ILoggingChannel2);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannelFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannelFactory[] = L"Windows.Foundation.Diagnostics.ILoggingChannelFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("4edc5b9c-af80-4a9b-b0dc-398f9ae5207b")
                ILoggingChannelFactory : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("This constructor creates a LoggingChannel in Windows 8.1 compatibility mode. Prefer the two-parameter constructor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING name,
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingChannelFactory = __uuidof(ILoggingChannelFactory);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannelFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannelFactory2[] = L"Windows.Foundation.Diagnostics.ILoggingChannelFactory2";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("4c6ef5dd-3b27-4dc9-99f0-299c6e4603a1")
                ILoggingChannelFactory2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithOptions(
                        HSTRING name,
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannelOptions* options,
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithOptionsAndId(
                        HSTRING name,
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannelOptions* options,
                        GUID id,
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingChannelFactory2 = __uuidof(ILoggingChannelFactory2);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannelOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannelOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannelOptions[] = L"Windows.Foundation.Diagnostics.ILoggingChannelOptions";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("c3e847ff-0ebb-4a53-8c54-dec24926cb2c")
                ILoggingChannelOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Group(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Group(
                        GUID value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingChannelOptions = __uuidof(ILoggingChannelOptions);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannelOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannelOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannelOptionsFactory[] = L"Windows.Foundation.Diagnostics.ILoggingChannelOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("a93151da-7faf-4191-8755-5e86dc65d896")
                ILoggingChannelOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        GUID group,
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannelOptions** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingChannelOptionsFactory = __uuidof(ILoggingChannelOptionsFactory);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingFields
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingFields
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingFields[] = L"Windows.Foundation.Diagnostics.ILoggingFields";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("d7f6b7af-762d-4579-83bd-52c23bc333bc")
                ILoggingFields : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BeginStruct(
                        HSTRING name
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BeginStructWithTags(
                        HSTRING name,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EndStruct(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddEmpty(
                        HSTRING name
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddEmptyWithFormat(
                        HSTRING name,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddEmptyWithFormatAndTags(
                        HSTRING name,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt8(
                        HSTRING name,
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt8WithFormat(
                        HSTRING name,
                        BYTE value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt8WithFormatAndTags(
                        HSTRING name,
                        BYTE value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt8Array(
                        HSTRING name,
                        UINT32 valueLength,
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt8ArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        BYTE* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt8ArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        BYTE* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt16(
                        HSTRING name,
                        INT16 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt16WithFormat(
                        HSTRING name,
                        INT16 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt16WithFormatAndTags(
                        HSTRING name,
                        INT16 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt16Array(
                        HSTRING name,
                        UINT32 valueLength,
                        INT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt16ArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        INT16* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt16ArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        INT16* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt16(
                        HSTRING name,
                        UINT16 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt16WithFormat(
                        HSTRING name,
                        UINT16 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt16WithFormatAndTags(
                        HSTRING name,
                        UINT16 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt16Array(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt16ArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT16* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt16ArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT16* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt32(
                        HSTRING name,
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt32WithFormat(
                        HSTRING name,
                        INT32 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt32WithFormatAndTags(
                        HSTRING name,
                        INT32 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt32Array(
                        HSTRING name,
                        UINT32 valueLength,
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt32ArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        INT32* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt32ArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        INT32* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt32(
                        HSTRING name,
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt32WithFormat(
                        HSTRING name,
                        UINT32 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt32WithFormatAndTags(
                        HSTRING name,
                        UINT32 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt32Array(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt32ArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT32* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt32ArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT32* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt64(
                        HSTRING name,
                        INT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt64WithFormat(
                        HSTRING name,
                        INT64 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt64WithFormatAndTags(
                        HSTRING name,
                        INT64 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt64Array(
                        HSTRING name,
                        UINT32 valueLength,
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt64ArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        INT64* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddInt64ArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        INT64* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt64(
                        HSTRING name,
                        UINT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt64WithFormat(
                        HSTRING name,
                        UINT64 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt64WithFormatAndTags(
                        HSTRING name,
                        UINT64 value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt64Array(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt64ArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT64* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddUInt64ArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        UINT64* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSingle(
                        HSTRING name,
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSingleWithFormat(
                        HSTRING name,
                        FLOAT value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSingleWithFormatAndTags(
                        HSTRING name,
                        FLOAT value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSingleArray(
                        HSTRING name,
                        UINT32 valueLength,
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSingleArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        FLOAT* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSingleArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        FLOAT* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDouble(
                        HSTRING name,
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDoubleWithFormat(
                        HSTRING name,
                        DOUBLE value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDoubleWithFormatAndTags(
                        HSTRING name,
                        DOUBLE value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDoubleArray(
                        HSTRING name,
                        UINT32 valueLength,
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDoubleArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        DOUBLE* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDoubleArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        DOUBLE* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddChar16(
                        HSTRING name,
                        WCHAR value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddChar16WithFormat(
                        HSTRING name,
                        WCHAR value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddChar16WithFormatAndTags(
                        HSTRING name,
                        WCHAR value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddChar16Array(
                        HSTRING name,
                        UINT32 valueLength,
                        WCHAR* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddChar16ArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        WCHAR* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddChar16ArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        WCHAR* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddBoolean(
                        HSTRING name,
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddBooleanWithFormat(
                        HSTRING name,
                        boolean value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddBooleanWithFormatAndTags(
                        HSTRING name,
                        boolean value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddBooleanArray(
                        HSTRING name,
                        UINT32 valueLength,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddBooleanArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        boolean* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddBooleanArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        boolean* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddString(
                        HSTRING name,
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddStringWithFormat(
                        HSTRING name,
                        HSTRING value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddStringWithFormatAndTags(
                        HSTRING name,
                        HSTRING value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddStringArray(
                        HSTRING name,
                        UINT32 valueLength,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddStringArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        HSTRING* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddStringArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        HSTRING* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddGuid(
                        HSTRING name,
                        GUID value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddGuidWithFormat(
                        HSTRING name,
                        GUID value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddGuidWithFormatAndTags(
                        HSTRING name,
                        GUID value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddGuidArray(
                        HSTRING name,
                        UINT32 valueLength,
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddGuidArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        GUID* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddGuidArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        GUID* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDateTime(
                        HSTRING name,
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDateTimeWithFormat(
                        HSTRING name,
                        ABI::Windows::Foundation::DateTime value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDateTimeWithFormatAndTags(
                        HSTRING name,
                        ABI::Windows::Foundation::DateTime value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDateTimeArray(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDateTimeArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::DateTime* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDateTimeArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::DateTime* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddTimeSpan(
                        HSTRING name,
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddTimeSpanWithFormat(
                        HSTRING name,
                        ABI::Windows::Foundation::TimeSpan value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddTimeSpanWithFormatAndTags(
                        HSTRING name,
                        ABI::Windows::Foundation::TimeSpan value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddTimeSpanArray(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddTimeSpanArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::TimeSpan* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddTimeSpanArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::TimeSpan* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddPoint(
                        HSTRING name,
                        ABI::Windows::Foundation::Point value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddPointWithFormat(
                        HSTRING name,
                        ABI::Windows::Foundation::Point value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddPointWithFormatAndTags(
                        HSTRING name,
                        ABI::Windows::Foundation::Point value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddPointArray(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddPointArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Point* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddPointArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Point* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSize(
                        HSTRING name,
                        ABI::Windows::Foundation::Size value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSizeWithFormat(
                        HSTRING name,
                        ABI::Windows::Foundation::Size value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSizeWithFormatAndTags(
                        HSTRING name,
                        ABI::Windows::Foundation::Size value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSizeArray(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSizeArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Size* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddSizeArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Size* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddRect(
                        HSTRING name,
                        ABI::Windows::Foundation::Rect value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddRectWithFormat(
                        HSTRING name,
                        ABI::Windows::Foundation::Rect value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddRectWithFormatAndTags(
                        HSTRING name,
                        ABI::Windows::Foundation::Rect value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddRectArray(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddRectArrayWithFormat(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Rect* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddRectArrayWithFormatAndTags(
                        HSTRING name,
                        UINT32 valueLength,
                        ABI::Windows::Foundation::Rect* value,
                        ABI::Windows::Foundation::Diagnostics::LoggingFieldFormat format,
                        INT32 tags
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingFields = __uuidof(ILoggingFields);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingOptions[] = L"Windows.Foundation.Diagnostics.ILoggingOptions";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("90bc7850-0192-4f5d-ac26-006adaca12d8")
                ILoggingOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Keywords(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Keywords(
                        INT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tags(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Tags(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Task(
                        INT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Task(
                        INT16 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Opcode(
                        ABI::Windows::Foundation::Diagnostics::LoggingOpcode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Opcode(
                        ABI::Windows::Foundation::Diagnostics::LoggingOpcode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ActivityId(
                        GUID value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RelatedActivityId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RelatedActivityId(
                        GUID value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingOptions = __uuidof(ILoggingOptions);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingOptionsFactory[] = L"Windows.Foundation.Diagnostics.ILoggingOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("d713c6cb-98ab-464b-9f22-a3268478368a")
                ILoggingOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithKeywords(
                        INT64 keywords,
                        ABI::Windows::Foundation::Diagnostics::ILoggingOptions** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingOptionsFactory = __uuidof(ILoggingOptionsFactory);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingSession[] = L"Windows.Foundation.Diagnostics.ILoggingSession";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("6221f306-9380-4ad7-baf5-41ea9310d768")
                ILoggingSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveToFileAsync(
                        ABI::Windows::Storage::IStorageFolder* folder,
                        HSTRING fileName,
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddLoggingChannel(
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel* loggingChannel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddLoggingChannelWithLevel(
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel* loggingChannel,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel maxLevel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveLoggingChannel(
                        ABI::Windows::Foundation::Diagnostics::ILoggingChannel* loggingChannel
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingSession = __uuidof(ILoggingSession);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingSessionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingSessionFactory[] = L"Windows.Foundation.Diagnostics.ILoggingSessionFactory";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("4e937ee5-58fd-45e0-8c2f-a132eff95c1e")
                ILoggingSessionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING name,
                        ABI::Windows::Foundation::Diagnostics::ILoggingSession** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingSessionFactory = __uuidof(ILoggingSessionFactory);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingTarget[] = L"Windows.Foundation.Diagnostics.ILoggingTarget";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("65f16c35-e388-4e26-b17a-f51cd3a83916")
                ILoggingTarget : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsEnabled(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsEnabledWithLevel(
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsEnabledWithLevelAndKeywords(
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level,
                        INT64 keywords,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogEvent(
                        HSTRING eventName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogEventWithFields(
                        HSTRING eventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingFields* fields
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogEventWithFieldsAndLevel(
                        HSTRING eventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingFields* fields,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogEventWithFieldsAndOptions(
                        HSTRING eventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingFields* fields,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level,
                        ABI::Windows::Foundation::Diagnostics::ILoggingOptions* options
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartActivity(
                        HSTRING startEventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingActivity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartActivityWithFields(
                        HSTRING startEventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingFields* fields,
                        ABI::Windows::Foundation::Diagnostics::ILoggingActivity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartActivityWithFieldsAndLevel(
                        HSTRING startEventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingFields* fields,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level,
                        ABI::Windows::Foundation::Diagnostics::ILoggingActivity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartActivityWithFieldsAndOptions(
                        HSTRING startEventName,
                        ABI::Windows::Foundation::Diagnostics::ILoggingFields* fields,
                        ABI::Windows::Foundation::Diagnostics::LoggingLevel level,
                        ABI::Windows::Foundation::Diagnostics::ILoggingOptions* options,
                        ABI::Windows::Foundation::Diagnostics::ILoggingActivity** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILoggingTarget = __uuidof(ILoggingTarget);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ITracingStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ITracingStatusChangedEventArgs[] = L"Windows.Foundation.Diagnostics.ITracingStatusChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Diagnostics {
                MIDL_INTERFACE("410b7711-ff3b-477f-9c9a-d2efda302dc3")
                ITracingStatusChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Enabled(
                        boolean* enabled
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TraceLevel(
                        ABI::Windows::Foundation::Diagnostics::CausalityTraceLevel* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITracingStatusChangedEventArgs = __uuidof(ITracingStatusChangedEventArgs);
            } /* Diagnostics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.AsyncCausalityTracer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.Diagnostics.IAsyncCausalityTracerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_AsyncCausalityTracer_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_AsyncCausalityTracer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_AsyncCausalityTracer[] = L"Windows.Foundation.Diagnostics.AsyncCausalityTracer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.ErrorDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.Diagnostics.IErrorDetailsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.IErrorDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_ErrorDetails_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_ErrorDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_ErrorDetails[] = L"Windows.Foundation.Diagnostics.ErrorDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.FileLoggingSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.IFileLoggingSessionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.IFileLoggingSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_FileLoggingSession_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_FileLoggingSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_FileLoggingSession[] = L"Windows.Foundation.Diagnostics.FileLoggingSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILogFileGeneratedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs[] = L"Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingActivity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingActivityFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingActivity ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.Diagnostics.ILoggingActivity2
 *    Windows.Foundation.Diagnostics.ILoggingTarget
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingActivity_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingActivity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingActivity[] = L"Windows.Foundation.Diagnostics.LoggingActivity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingChannelFactory2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingChannelFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingChannel ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.Diagnostics.ILoggingChannel2
 *    Windows.Foundation.Diagnostics.ILoggingTarget
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingChannel_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingChannel[] = L"Windows.Foundation.Diagnostics.LoggingChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingChannelOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingChannelOptionsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingChannelOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingChannelOptions_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingChannelOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingChannelOptions[] = L"Windows.Foundation.Diagnostics.LoggingChannelOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingFields
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingFields ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingFields_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingFields_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingFields[] = L"Windows.Foundation.Diagnostics.LoggingFields";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingOptionsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingOptions_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingOptions[] = L"Windows.Foundation.Diagnostics.LoggingOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingSessionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingSession_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingSession[] = L"Windows.Foundation.Diagnostics.LoggingSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.IErrorReportingSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_RuntimeBrokerErrorSettings_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_RuntimeBrokerErrorSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_RuntimeBrokerErrorSettings[] = L"Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ITracingStatusChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs[] = L"Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2 __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2 __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2 __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs;

#endif // ____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails;

typedef struct __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetailsVtbl;

interface __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails* This,
        __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetailsVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CDiagnostics__CErrorDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs;

typedef struct __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgsVtbl;

interface __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* sender,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityRelation __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityRelation;

typedef enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySource __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySource;

typedef enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySynchronousWork __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySynchronousWork;

typedef enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel;

typedef enum __x_ABI_CWindows_CFoundation_CDiagnostics_CErrorOptions __x_ABI_CWindows_CFoundation_CDiagnostics_CErrorOptions;

typedef enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat;

typedef enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel;

typedef enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingOpcode __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingOpcode;

/*
 *
 * Struct Windows.Foundation.Diagnostics.CausalityRelation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityRelation
{
    CausalityRelation_AssignDelegate = 0,
    CausalityRelation_Join = 1,
    CausalityRelation_Choice = 2,
    CausalityRelation_Cancel = 3,
    CausalityRelation_Error = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.CausalitySource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySource
{
    CausalitySource_Application = 0,
    CausalitySource_Library = 1,
    CausalitySource_System = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.CausalitySynchronousWork
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySynchronousWork
{
    CausalitySynchronousWork_CompletionNotification = 0,
    CausalitySynchronousWork_ProgressNotification = 1,
    CausalitySynchronousWork_Execution = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.CausalityTraceLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel
{
    CausalityTraceLevel_Required = 0,
    CausalityTraceLevel_Important = 1,
    CausalityTraceLevel_Verbose = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.ErrorOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CDiagnostics_CErrorOptions
{
    ErrorOptions_None = 0,
    ErrorOptions_SuppressExceptions = 0x1,
    ErrorOptions_ForceExceptions = 0x2,
    ErrorOptions_UseSetErrorInfo = 0x4,
    ErrorOptions_SuppressSetErrorInfo = 0x8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.LoggingFieldFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat
{
    LoggingFieldFormat_Default = 0,
    LoggingFieldFormat_Hidden = 1,
    LoggingFieldFormat_String = 2,
    LoggingFieldFormat_Boolean = 3,
    LoggingFieldFormat_Hexadecimal = 4,
    LoggingFieldFormat_ProcessId = 5,
    LoggingFieldFormat_ThreadId = 6,
    LoggingFieldFormat_Port = 7,
    LoggingFieldFormat_Ipv4Address = 8,
    LoggingFieldFormat_Ipv6Address = 9,
    LoggingFieldFormat_SocketAddress = 10,
    LoggingFieldFormat_Xml = 11,
    LoggingFieldFormat_Json = 12,
    LoggingFieldFormat_Win32Error = 13,
    LoggingFieldFormat_NTStatus = 14,
    LoggingFieldFormat_HResult = 15,
    LoggingFieldFormat_FileTime = 16,
    LoggingFieldFormat_Signed = 17,
    LoggingFieldFormat_Unsigned = 18,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.LoggingLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel
{
    LoggingLevel_Verbose = 0,
    LoggingLevel_Information = 1,
    LoggingLevel_Warning = 2,
    LoggingLevel_Error = 3,
    LoggingLevel_Critical = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Foundation.Diagnostics.LoggingOpcode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingOpcode
{
    LoggingOpcode_Info = 0,
    LoggingOpcode_Start = 1,
    LoggingOpcode_Stop = 2,
    LoggingOpcode_Reply = 6,
    LoggingOpcode_Resume = 7,
    LoggingOpcode_Suspend = 8,
    LoggingOpcode_Send = 9,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IAsyncCausalityTracerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.AsyncCausalityTracer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IAsyncCausalityTracerStatics[] = L"Windows.Foundation.Diagnostics.IAsyncCausalityTracerStatics";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TraceOperationCreation)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel traceLevel,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySource source,
        GUID platformId,
        UINT64 operationId,
        HSTRING operationName,
        UINT64 relatedContext);
    HRESULT (STDMETHODCALLTYPE* TraceOperationCompletion)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel traceLevel,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySource source,
        GUID platformId,
        UINT64 operationId,
        AsyncStatus status);
    HRESULT (STDMETHODCALLTYPE* TraceOperationRelation)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel traceLevel,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySource source,
        GUID platformId,
        UINT64 operationId,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityRelation relation);
    HRESULT (STDMETHODCALLTYPE* TraceSynchronousWorkStart)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel traceLevel,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySource source,
        GUID platformId,
        UINT64 operationId,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySynchronousWork work);
    HRESULT (STDMETHODCALLTYPE* TraceSynchronousWorkCompletion)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel traceLevel,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySource source,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalitySynchronousWork work);
    HRESULT (STDMETHODCALLTYPE* add_TracingStatusChanged)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        __FIEventHandler_1_Windows__CFoundation__CDiagnostics__CTracingStatusChangedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_TracingStatusChanged)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStaticsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_TraceOperationCreation(This, traceLevel, source, platformId, operationId, operationName, relatedContext) \
    ((This)->lpVtbl->TraceOperationCreation(This, traceLevel, source, platformId, operationId, operationName, relatedContext))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_TraceOperationCompletion(This, traceLevel, source, platformId, operationId, status) \
    ((This)->lpVtbl->TraceOperationCompletion(This, traceLevel, source, platformId, operationId, status))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_TraceOperationRelation(This, traceLevel, source, platformId, operationId, relation) \
    ((This)->lpVtbl->TraceOperationRelation(This, traceLevel, source, platformId, operationId, relation))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_TraceSynchronousWorkStart(This, traceLevel, source, platformId, operationId, work) \
    ((This)->lpVtbl->TraceSynchronousWorkStart(This, traceLevel, source, platformId, operationId, work))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_TraceSynchronousWorkCompletion(This, traceLevel, source, work) \
    ((This)->lpVtbl->TraceSynchronousWorkCompletion(This, traceLevel, source, work))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_add_TracingStatusChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_TracingStatusChanged(This, handler, cookie))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_remove_TracingStatusChanged(This, cookie) \
    ((This)->lpVtbl->remove_TracingStatusChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIAsyncCausalityTracerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IErrorDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.ErrorDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IErrorDetails[] = L"Windows.Foundation.Diagnostics.IErrorDetails";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LongDescription)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HelpUri)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_get_LongDescription(This, value) \
    ((This)->lpVtbl->get_LongDescription(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_get_HelpUri(This, value) \
    ((This)->lpVtbl->get_HelpUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IErrorDetailsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.ErrorDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IErrorDetailsStatics[] = L"Windows.Foundation.Diagnostics.IErrorDetailsStatics";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromHResultAsync)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics* This,
        INT32 errorCode,
        __FIAsyncOperation_1_Windows__CFoundation__CDiagnostics__CErrorDetails** operation);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStaticsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_CreateFromHResultAsync(This, errorCode, operation) \
    ((This)->lpVtbl->CreateFromHResultAsync(This, errorCode, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorDetailsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IErrorReportingSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IErrorReportingSettings[] = L"Windows.Foundation.Diagnostics.IErrorReportingSettings";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetErrorOptions)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CErrorOptions value);
    HRESULT (STDMETHODCALLTYPE* GetErrorOptions)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CErrorOptions* value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettingsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_SetErrorOptions(This, value) \
    ((This)->lpVtbl->SetErrorOptions(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_GetErrorOptions(This, value) \
    ((This)->lpVtbl->GetErrorOptions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIErrorReportingSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IFileLoggingSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IFileLoggingSession[] = L"Windows.Foundation.Diagnostics.IFileLoggingSession";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* AddLoggingChannel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* loggingChannel);
    HRESULT (STDMETHODCALLTYPE* AddLoggingChannelWithLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* loggingChannel,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel maxLevel);
    HRESULT (STDMETHODCALLTYPE* RemoveLoggingChannel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* loggingChannel);
    HRESULT (STDMETHODCALLTYPE* CloseAndSaveToFileAsync)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* add_LogFileGenerated)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CIFileLoggingSession_Windows__CFoundation__CDiagnostics__CLogFileGeneratedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LogFileGenerated)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_AddLoggingChannel(This, loggingChannel) \
    ((This)->lpVtbl->AddLoggingChannel(This, loggingChannel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_AddLoggingChannelWithLevel(This, loggingChannel, maxLevel) \
    ((This)->lpVtbl->AddLoggingChannelWithLevel(This, loggingChannel, maxLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_RemoveLoggingChannel(This, loggingChannel) \
    ((This)->lpVtbl->RemoveLoggingChannel(This, loggingChannel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_CloseAndSaveToFileAsync(This, operation) \
    ((This)->lpVtbl->CloseAndSaveToFileAsync(This, operation))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_add_LogFileGenerated(This, handler, token) \
    ((This)->lpVtbl->add_LogFileGenerated(This, handler, token))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_remove_LogFileGenerated(This, token) \
    ((This)->lpVtbl->remove_LogFileGenerated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.IFileLoggingSessionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.FileLoggingSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_IFileLoggingSessionFactory[] = L"Windows.Foundation.Diagnostics.IFileLoggingSessionFactory";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSession** result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_Create(This, name, result) \
    ((This)->lpVtbl->Create(This, name, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CIFileLoggingSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILogFileGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILogFileGeneratedEventArgs[] = L"Windows.Foundation.Diagnostics.ILogFileGeneratedEventArgs";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_File)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_get_File(This, value) \
    ((This)->lpVtbl->get_File(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILogFileGeneratedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingActivity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingActivity
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingActivity[] = L"Windows.Foundation.Diagnostics.ILoggingActivity";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingActivity2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingActivity
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Diagnostics.ILoggingActivity
 *     Windows.Foundation.IClosable
 *     Windows.Foundation.Diagnostics.ILoggingTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingActivity2[] = L"Windows.Foundation.Diagnostics.ILoggingActivity2";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Channel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel** value);
    HRESULT (STDMETHODCALLTYPE* StopActivity)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This,
        HSTRING stopEventName);
    HRESULT (STDMETHODCALLTYPE* StopActivityWithFields)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This,
        HSTRING stopEventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* fields);
    HRESULT (STDMETHODCALLTYPE* StopActivityWithFieldsAndOptions)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2* This,
        HSTRING stopEventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* fields,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* options);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2Vtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_get_Channel(This, value) \
    ((This)->lpVtbl->get_Channel(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_StopActivity(This, stopEventName) \
    ((This)->lpVtbl->StopActivity(This, stopEventName))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_StopActivityWithFields(This, stopEventName, fields) \
    ((This)->lpVtbl->StopActivityWithFields(This, stopEventName, fields))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_StopActivityWithFieldsAndOptions(This, stopEventName, fields, options) \
    ((This)->lpVtbl->StopActivityWithFieldsAndOptions(This, stopEventName, fields, options))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingActivityFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingActivity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingActivityFactory[] = L"Windows.Foundation.Diagnostics.ILoggingActivityFactory";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateLoggingActivity)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory* This,
        HSTRING activityName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* loggingChannel,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity** loggingActivity);
    HRESULT (STDMETHODCALLTYPE* CreateLoggingActivityWithLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory* This,
        HSTRING activityName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* loggingChannel,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity** loggingActivity);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_CreateLoggingActivity(This, activityName, loggingChannel, loggingActivity) \
    ((This)->lpVtbl->CreateLoggingActivity(This, activityName, loggingChannel, loggingActivity))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_CreateLoggingActivityWithLevel(This, activityName, loggingChannel, level, loggingActivity) \
    ((This)->lpVtbl->CreateLoggingActivityWithLevel(This, activityName, loggingChannel, level, loggingActivity))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivityFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannel[] = L"Windows.Foundation.Diagnostics.ILoggingChannel";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Level)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel* value);
    HRESULT (STDMETHODCALLTYPE* LogMessage)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        HSTRING eventString);
    HRESULT (STDMETHODCALLTYPE* LogMessageWithLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        HSTRING eventString,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level);
    HRESULT (STDMETHODCALLTYPE* LogValuePair)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        HSTRING value1,
        INT32 value2);
    HRESULT (STDMETHODCALLTYPE* LogValuePairWithLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        HSTRING value1,
        INT32 value2,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level);
    HRESULT (STDMETHODCALLTYPE* add_LoggingEnabled)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        __FITypedEventHandler_2_Windows__CFoundation__CDiagnostics__CILoggingChannel_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LoggingEnabled)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_get_Enabled(This, value) \
    ((This)->lpVtbl->get_Enabled(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_get_Level(This, value) \
    ((This)->lpVtbl->get_Level(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_LogMessage(This, eventString) \
    ((This)->lpVtbl->LogMessage(This, eventString))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_LogMessageWithLevel(This, eventString, level) \
    ((This)->lpVtbl->LogMessageWithLevel(This, eventString, level))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_LogValuePair(This, value1, value2) \
    ((This)->lpVtbl->LogValuePair(This, value1, value2))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_LogValuePairWithLevel(This, value1, value2, level) \
    ((This)->lpVtbl->LogValuePairWithLevel(This, value1, value2, level))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_add_LoggingEnabled(This, handler, token) \
    ((This)->lpVtbl->add_LoggingEnabled(This, handler, token))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_remove_LoggingEnabled(This, token) \
    ((This)->lpVtbl->remove_LoggingEnabled(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannel2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannel
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Diagnostics.ILoggingChannel
 *     Windows.Foundation.IClosable
 *     Windows.Foundation.Diagnostics.ILoggingTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannel2[] = L"Windows.Foundation.Diagnostics.ILoggingChannel2";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2Vtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannelFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannelFactory[] = L"Windows.Foundation.Diagnostics.ILoggingChannelFactory";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("This constructor creates a LoggingChannel in Windows 8.1 compatibility mode. Prefer the two-parameter constructor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel** result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("This constructor creates a LoggingChannel in Windows 8.1 compatibility mode. Prefer the two-parameter constructor.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_Create(This, name, result) \
    ((This)->lpVtbl->Create(This, name, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannelFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannelFactory2[] = L"Windows.Foundation.Diagnostics.ILoggingChannelFactory2";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithOptions)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2* This,
        HSTRING name,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* options,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithOptionsAndId)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2* This,
        HSTRING name,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* options,
        GUID id,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel** result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2Vtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_CreateWithOptions(This, name, options, result) \
    ((This)->lpVtbl->CreateWithOptions(This, name, options, result))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_CreateWithOptionsAndId(This, name, options, id, result) \
    ((This)->lpVtbl->CreateWithOptionsAndId(This, name, options, id, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannelOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannelOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannelOptions[] = L"Windows.Foundation.Diagnostics.ILoggingChannelOptions";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Group)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_Group)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions* This,
        GUID value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_get_Group(This, value) \
    ((This)->lpVtbl->get_Group(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_put_Group(This, value) \
    ((This)->lpVtbl->put_Group(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingChannelOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingChannelOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingChannelOptionsFactory[] = L"Windows.Foundation.Diagnostics.ILoggingChannelOptionsFactory";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory* This,
        GUID group,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptions** result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_Create(This, group, result) \
    ((This)->lpVtbl->Create(This, group, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannelOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingFields
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingFields
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingFields[] = L"Windows.Foundation.Diagnostics.ILoggingFields";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFieldsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This);
    HRESULT (STDMETHODCALLTYPE* BeginStruct)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name);
    HRESULT (STDMETHODCALLTYPE* BeginStructWithTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* EndStruct)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This);
    HRESULT (STDMETHODCALLTYPE* AddEmpty)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name);
    HRESULT (STDMETHODCALLTYPE* AddEmptyWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddEmptyWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddUInt8)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* AddUInt8WithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        BYTE value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddUInt8WithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        BYTE value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddUInt8Array)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* AddUInt8ArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        BYTE* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddUInt8ArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        BYTE* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddInt16)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT16 value);
    HRESULT (STDMETHODCALLTYPE* AddInt16WithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT16 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddInt16WithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT16 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddInt16Array)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT16* value);
    HRESULT (STDMETHODCALLTYPE* AddInt16ArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT16* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddInt16ArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT16* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddUInt16)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT16 value);
    HRESULT (STDMETHODCALLTYPE* AddUInt16WithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT16 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddUInt16WithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT16 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddUInt16Array)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* AddUInt16ArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT16* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddUInt16ArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT16* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddInt32)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* AddInt32WithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT32 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddInt32WithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT32 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddInt32Array)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* AddInt32ArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT32* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddInt32ArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT32* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddUInt32)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* AddUInt32WithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddUInt32WithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddUInt32Array)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* AddUInt32ArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT32* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddUInt32ArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT32* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddInt64)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT64 value);
    HRESULT (STDMETHODCALLTYPE* AddInt64WithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT64 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddInt64WithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        INT64 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddInt64Array)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* AddInt64ArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT64* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddInt64ArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        INT64* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddUInt64)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT64 value);
    HRESULT (STDMETHODCALLTYPE* AddUInt64WithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT64 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddUInt64WithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT64 value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddUInt64Array)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* AddUInt64ArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT64* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddUInt64ArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        UINT64* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddSingle)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* AddSingleWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        FLOAT value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddSingleWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        FLOAT value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddSingleArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* AddSingleArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        FLOAT* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddSingleArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        FLOAT* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddDouble)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* AddDoubleWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        DOUBLE value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddDoubleWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        DOUBLE value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddDoubleArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* AddDoubleArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        DOUBLE* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddDoubleArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        DOUBLE* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddChar16)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        WCHAR value);
    HRESULT (STDMETHODCALLTYPE* AddChar16WithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        WCHAR value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddChar16WithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        WCHAR value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddChar16Array)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        WCHAR* value);
    HRESULT (STDMETHODCALLTYPE* AddChar16ArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        WCHAR* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddChar16ArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        WCHAR* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddBoolean)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* AddBooleanWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        boolean value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddBooleanWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        boolean value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddBooleanArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* AddBooleanArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        boolean* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddBooleanArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        boolean* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddString)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* AddStringWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        HSTRING value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddStringWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        HSTRING value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddStringArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* AddStringArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        HSTRING* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddStringArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        HSTRING* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddGuid)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* AddGuidWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        GUID value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddGuidWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        GUID value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddGuidArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* AddGuidArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        GUID* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddGuidArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        GUID* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddDateTime)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* AddDateTimeWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CDateTime value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddDateTimeWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CDateTime value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddDateTimeArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* AddDateTimeArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddDateTimeArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddTimeSpan)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* AddTimeSpanWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddTimeSpanWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddTimeSpanArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* AddTimeSpanArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddTimeSpanArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddPoint)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CPoint value);
    HRESULT (STDMETHODCALLTYPE* AddPointWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CPoint value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddPointWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CPoint value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddPointArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* AddPointArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddPointArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddSize)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CSize value);
    HRESULT (STDMETHODCALLTYPE* AddSizeWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CSize value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddSizeWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CSize value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddSizeArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* AddSizeArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CSize* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddSizeArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CSize* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddRect)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CRect value);
    HRESULT (STDMETHODCALLTYPE* AddRectWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CRect value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddRectWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        struct __x_ABI_CWindows_CFoundation_CRect value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);
    HRESULT (STDMETHODCALLTYPE* AddRectArray)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* AddRectArrayWithFormat)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CRect* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format);
    HRESULT (STDMETHODCALLTYPE* AddRectArrayWithFormatAndTags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* This,
        HSTRING name,
        UINT32 valueLength,
        struct __x_ABI_CWindows_CFoundation_CRect* value,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingFieldFormat format,
        INT32 tags);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFieldsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFieldsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_BeginStruct(This, name) \
    ((This)->lpVtbl->BeginStruct(This, name))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_BeginStructWithTags(This, name, tags) \
    ((This)->lpVtbl->BeginStructWithTags(This, name, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_EndStruct(This) \
    ((This)->lpVtbl->EndStruct(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddEmpty(This, name) \
    ((This)->lpVtbl->AddEmpty(This, name))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddEmptyWithFormat(This, name, format) \
    ((This)->lpVtbl->AddEmptyWithFormat(This, name, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddEmptyWithFormatAndTags(This, name, format, tags) \
    ((This)->lpVtbl->AddEmptyWithFormatAndTags(This, name, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt8(This, name, value) \
    ((This)->lpVtbl->AddUInt8(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt8WithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddUInt8WithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt8WithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddUInt8WithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt8Array(This, name, valueLength, value) \
    ((This)->lpVtbl->AddUInt8Array(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt8ArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddUInt8ArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt8ArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddUInt8ArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt16(This, name, value) \
    ((This)->lpVtbl->AddInt16(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt16WithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddInt16WithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt16WithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddInt16WithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt16Array(This, name, valueLength, value) \
    ((This)->lpVtbl->AddInt16Array(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt16ArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddInt16ArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt16ArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddInt16ArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt16(This, name, value) \
    ((This)->lpVtbl->AddUInt16(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt16WithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddUInt16WithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt16WithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddUInt16WithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt16Array(This, name, valueLength, value) \
    ((This)->lpVtbl->AddUInt16Array(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt16ArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddUInt16ArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt16ArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddUInt16ArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt32(This, name, value) \
    ((This)->lpVtbl->AddInt32(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt32WithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddInt32WithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt32WithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddInt32WithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt32Array(This, name, valueLength, value) \
    ((This)->lpVtbl->AddInt32Array(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt32ArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddInt32ArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt32ArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddInt32ArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt32(This, name, value) \
    ((This)->lpVtbl->AddUInt32(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt32WithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddUInt32WithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt32WithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddUInt32WithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt32Array(This, name, valueLength, value) \
    ((This)->lpVtbl->AddUInt32Array(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt32ArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddUInt32ArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt32ArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddUInt32ArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt64(This, name, value) \
    ((This)->lpVtbl->AddInt64(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt64WithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddInt64WithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt64WithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddInt64WithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt64Array(This, name, valueLength, value) \
    ((This)->lpVtbl->AddInt64Array(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt64ArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddInt64ArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddInt64ArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddInt64ArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt64(This, name, value) \
    ((This)->lpVtbl->AddUInt64(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt64WithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddUInt64WithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt64WithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddUInt64WithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt64Array(This, name, valueLength, value) \
    ((This)->lpVtbl->AddUInt64Array(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt64ArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddUInt64ArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddUInt64ArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddUInt64ArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSingle(This, name, value) \
    ((This)->lpVtbl->AddSingle(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSingleWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddSingleWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSingleWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddSingleWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSingleArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddSingleArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSingleArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddSingleArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSingleArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddSingleArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDouble(This, name, value) \
    ((This)->lpVtbl->AddDouble(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDoubleWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddDoubleWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDoubleWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddDoubleWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDoubleArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddDoubleArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDoubleArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddDoubleArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDoubleArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddDoubleArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddChar16(This, name, value) \
    ((This)->lpVtbl->AddChar16(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddChar16WithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddChar16WithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddChar16WithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddChar16WithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddChar16Array(This, name, valueLength, value) \
    ((This)->lpVtbl->AddChar16Array(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddChar16ArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddChar16ArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddChar16ArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddChar16ArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddBoolean(This, name, value) \
    ((This)->lpVtbl->AddBoolean(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddBooleanWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddBooleanWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddBooleanWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddBooleanWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddBooleanArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddBooleanArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddBooleanArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddBooleanArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddBooleanArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddBooleanArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddString(This, name, value) \
    ((This)->lpVtbl->AddString(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddStringWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddStringWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddStringWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddStringWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddStringArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddStringArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddStringArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddStringArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddStringArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddStringArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddGuid(This, name, value) \
    ((This)->lpVtbl->AddGuid(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddGuidWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddGuidWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddGuidWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddGuidWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddGuidArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddGuidArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddGuidArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddGuidArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddGuidArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddGuidArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDateTime(This, name, value) \
    ((This)->lpVtbl->AddDateTime(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDateTimeWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddDateTimeWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDateTimeWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddDateTimeWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDateTimeArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddDateTimeArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDateTimeArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddDateTimeArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddDateTimeArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddDateTimeArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddTimeSpan(This, name, value) \
    ((This)->lpVtbl->AddTimeSpan(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddTimeSpanWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddTimeSpanWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddTimeSpanWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddTimeSpanWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddTimeSpanArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddTimeSpanArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddTimeSpanArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddTimeSpanArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddTimeSpanArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddTimeSpanArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddPoint(This, name, value) \
    ((This)->lpVtbl->AddPoint(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddPointWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddPointWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddPointWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddPointWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddPointArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddPointArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddPointArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddPointArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddPointArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddPointArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSize(This, name, value) \
    ((This)->lpVtbl->AddSize(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSizeWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddSizeWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSizeWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddSizeWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSizeArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddSizeArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSizeArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddSizeArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddSizeArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddSizeArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddRect(This, name, value) \
    ((This)->lpVtbl->AddRect(This, name, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddRectWithFormat(This, name, value, format) \
    ((This)->lpVtbl->AddRectWithFormat(This, name, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddRectWithFormatAndTags(This, name, value, format, tags) \
    ((This)->lpVtbl->AddRectWithFormatAndTags(This, name, value, format, tags))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddRectArray(This, name, valueLength, value) \
    ((This)->lpVtbl->AddRectArray(This, name, valueLength, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddRectArrayWithFormat(This, name, valueLength, value, format) \
    ((This)->lpVtbl->AddRectArrayWithFormat(This, name, valueLength, value, format))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_AddRectArrayWithFormatAndTags(This, name, valueLength, value, format, tags) \
    ((This)->lpVtbl->AddRectArrayWithFormatAndTags(This, name, valueLength, value, format, tags))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingOptions[] = L"Windows.Foundation.Diagnostics.ILoggingOptions";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Keywords)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* put_Keywords)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        INT64 value);
    HRESULT (STDMETHODCALLTYPE* get_Tags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Tags)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Task)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        INT16* value);
    HRESULT (STDMETHODCALLTYPE* put_Task)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        INT16 value);
    HRESULT (STDMETHODCALLTYPE* get_Opcode)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingOpcode* value);
    HRESULT (STDMETHODCALLTYPE* put_Opcode)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingOpcode value);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_ActivityId)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* get_RelatedActivityId)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_RelatedActivityId)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* This,
        GUID value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_get_Keywords(This, value) \
    ((This)->lpVtbl->get_Keywords(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_put_Keywords(This, value) \
    ((This)->lpVtbl->put_Keywords(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_get_Tags(This, value) \
    ((This)->lpVtbl->get_Tags(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_put_Tags(This, value) \
    ((This)->lpVtbl->put_Tags(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_get_Task(This, value) \
    ((This)->lpVtbl->get_Task(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_put_Task(This, value) \
    ((This)->lpVtbl->put_Task(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_get_Opcode(This, value) \
    ((This)->lpVtbl->get_Opcode(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_put_Opcode(This, value) \
    ((This)->lpVtbl->put_Opcode(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_put_ActivityId(This, value) \
    ((This)->lpVtbl->put_ActivityId(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_get_RelatedActivityId(This, value) \
    ((This)->lpVtbl->get_RelatedActivityId(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_put_RelatedActivityId(This, value) \
    ((This)->lpVtbl->put_RelatedActivityId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingOptionsFactory[] = L"Windows.Foundation.Diagnostics.ILoggingOptionsFactory";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithKeywords)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory* This,
        INT64 keywords,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions** result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_CreateWithKeywords(This, keywords, result) \
    ((This)->lpVtbl->CreateWithKeywords(This, keywords, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingSession[] = L"Windows.Foundation.Diagnostics.ILoggingSession";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* SaveToFileAsync)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* folder,
        HSTRING fileName,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* AddLoggingChannel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* loggingChannel);
    HRESULT (STDMETHODCALLTYPE* AddLoggingChannelWithLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* loggingChannel,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel maxLevel);
    HRESULT (STDMETHODCALLTYPE* RemoveLoggingChannel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession* This,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingChannel* loggingChannel);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_SaveToFileAsync(This, folder, fileName, operation) \
    ((This)->lpVtbl->SaveToFileAsync(This, folder, fileName, operation))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_AddLoggingChannel(This, loggingChannel) \
    ((This)->lpVtbl->AddLoggingChannel(This, loggingChannel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_AddLoggingChannelWithLevel(This, loggingChannel, maxLevel) \
    ((This)->lpVtbl->AddLoggingChannelWithLevel(This, loggingChannel, maxLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_RemoveLoggingChannel(This, loggingChannel) \
    ((This)->lpVtbl->RemoveLoggingChannel(This, loggingChannel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingSessionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.LoggingSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingSessionFactory[] = L"Windows.Foundation.Diagnostics.ILoggingSessionFactory";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSession** result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactoryVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_Create(This, name, result) \
    ((This)->lpVtbl->Create(This, name, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ILoggingTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ILoggingTarget[] = L"Windows.Foundation.Diagnostics.ILoggingTarget";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTargetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsEnabled)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsEnabledWithLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsEnabledWithLevelAndKeywords)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level,
        INT64 keywords,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* LogEvent)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING eventName);
    HRESULT (STDMETHODCALLTYPE* LogEventWithFields)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING eventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* fields);
    HRESULT (STDMETHODCALLTYPE* LogEventWithFieldsAndLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING eventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* fields,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level);
    HRESULT (STDMETHODCALLTYPE* LogEventWithFieldsAndOptions)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING eventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* fields,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* options);
    HRESULT (STDMETHODCALLTYPE* StartActivity)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING startEventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity** result);
    HRESULT (STDMETHODCALLTYPE* StartActivityWithFields)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING startEventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* fields,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity** result);
    HRESULT (STDMETHODCALLTYPE* StartActivityWithFieldsAndLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING startEventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* fields,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity** result);
    HRESULT (STDMETHODCALLTYPE* StartActivityWithFieldsAndOptions)(__x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget* This,
        HSTRING startEventName,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingFields* fields,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CLoggingLevel level,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingOptions* options,
        __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingActivity** result);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTargetVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTargetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_IsEnabled(This, result) \
    ((This)->lpVtbl->IsEnabled(This, result))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_IsEnabledWithLevel(This, level, result) \
    ((This)->lpVtbl->IsEnabledWithLevel(This, level, result))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_IsEnabledWithLevelAndKeywords(This, level, keywords, result) \
    ((This)->lpVtbl->IsEnabledWithLevelAndKeywords(This, level, keywords, result))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_LogEvent(This, eventName) \
    ((This)->lpVtbl->LogEvent(This, eventName))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_LogEventWithFields(This, eventName, fields) \
    ((This)->lpVtbl->LogEventWithFields(This, eventName, fields))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_LogEventWithFieldsAndLevel(This, eventName, fields, level) \
    ((This)->lpVtbl->LogEventWithFieldsAndLevel(This, eventName, fields, level))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_LogEventWithFieldsAndOptions(This, eventName, fields, level, options) \
    ((This)->lpVtbl->LogEventWithFieldsAndOptions(This, eventName, fields, level, options))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_StartActivity(This, startEventName, result) \
    ((This)->lpVtbl->StartActivity(This, startEventName, result))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_StartActivityWithFields(This, startEventName, fields, result) \
    ((This)->lpVtbl->StartActivityWithFields(This, startEventName, fields, result))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_StartActivityWithFieldsAndLevel(This, startEventName, fields, level, result) \
    ((This)->lpVtbl->StartActivityWithFieldsAndLevel(This, startEventName, fields, level, result))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_StartActivityWithFieldsAndOptions(This, startEventName, fields, level, options, result) \
    ((This)->lpVtbl->StartActivityWithFieldsAndOptions(This, startEventName, fields, level, options, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CILoggingTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Foundation.Diagnostics.ITracingStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_Diagnostics_ITracingStatusChangedEventArgs[] = L"Windows.Foundation.Diagnostics.ITracingStatusChangedEventArgs";
typedef struct __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Enabled)(__x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* This,
        boolean* enabled);
    HRESULT (STDMETHODCALLTYPE* get_TraceLevel)(__x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs* This,
        enum __x_ABI_CWindows_CFoundation_CDiagnostics_CCausalityTraceLevel* value);

    END_INTERFACE
} __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgsVtbl;

interface __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_get_Enabled(This, enabled) \
    ((This)->lpVtbl->get_Enabled(This, enabled))

#define __x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_get_TraceLevel(This, value) \
    ((This)->lpVtbl->get_TraceLevel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CDiagnostics_CITracingStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.AsyncCausalityTracer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.Diagnostics.IAsyncCausalityTracerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_AsyncCausalityTracer_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_AsyncCausalityTracer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_AsyncCausalityTracer[] = L"Windows.Foundation.Diagnostics.AsyncCausalityTracer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.ErrorDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Foundation.Diagnostics.IErrorDetailsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.IErrorDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_ErrorDetails_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_ErrorDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_ErrorDetails[] = L"Windows.Foundation.Diagnostics.ErrorDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.FileLoggingSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.IFileLoggingSessionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.IFileLoggingSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_FileLoggingSession_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_FileLoggingSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_FileLoggingSession[] = L"Windows.Foundation.Diagnostics.FileLoggingSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILogFileGeneratedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs[] = L"Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingActivity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingActivityFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingActivity ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.Diagnostics.ILoggingActivity2
 *    Windows.Foundation.Diagnostics.ILoggingTarget
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingActivity_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingActivity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingActivity[] = L"Windows.Foundation.Diagnostics.LoggingActivity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingChannelFactory2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingChannelFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingChannel ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.Diagnostics.ILoggingChannel2
 *    Windows.Foundation.Diagnostics.ILoggingTarget
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingChannel_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingChannel[] = L"Windows.Foundation.Diagnostics.LoggingChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingChannelOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingChannelOptionsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingChannelOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingChannelOptions_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingChannelOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingChannelOptions[] = L"Windows.Foundation.Diagnostics.LoggingChannelOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingFields
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingFields ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingFields_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingFields_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingFields[] = L"Windows.Foundation.Diagnostics.LoggingFields";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingOptionsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingOptions_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingOptions[] = L"Windows.Foundation.Diagnostics.LoggingOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.LoggingSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Foundation.Diagnostics.ILoggingSessionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ILoggingSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingSession_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_LoggingSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_LoggingSession[] = L"Windows.Foundation.Diagnostics.LoggingSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.IErrorReportingSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_RuntimeBrokerErrorSettings_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_RuntimeBrokerErrorSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_RuntimeBrokerErrorSettings[] = L"Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Diagnostics.ITracingStatusChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs[] = L"Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
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
#endif // __windows2Efoundation2Ediagnostics_p_h__

#endif // __windows2Efoundation2Ediagnostics_h__
