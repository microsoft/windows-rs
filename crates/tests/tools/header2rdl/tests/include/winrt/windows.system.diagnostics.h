
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
#ifndef __windows2Esystem2Ediagnostics_h__
#define __windows2Esystem2Ediagnostics_h__
#ifndef __windows2Esystem2Ediagnostics_p_h__
#define __windows2Esystem2Ediagnostics_p_h__


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
#include "Windows.Data.Json.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IDiagnosticActionResult;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult ABI::Windows::System::Diagnostics::IDiagnosticActionResult

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IDiagnosticInvoker;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker ABI::Windows::System::Diagnostics::IDiagnosticInvoker

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IDiagnosticInvoker2;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2 ABI::Windows::System::Diagnostics::IDiagnosticInvoker2

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IDiagnosticInvokerStatics;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics ABI::Windows::System::Diagnostics::IDiagnosticInvokerStatics

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessCpuUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage ABI::Windows::System::Diagnostics::IProcessCpuUsage

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessCpuUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport ABI::Windows::System::Diagnostics::IProcessCpuUsageReport

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessDiagnosticInfo;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessDiagnosticInfo2;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2 ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo2

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessDiagnosticInfoStatics;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics ABI::Windows::System::Diagnostics::IProcessDiagnosticInfoStatics

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessDiagnosticInfoStatics2;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2 ABI::Windows::System::Diagnostics::IProcessDiagnosticInfoStatics2

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessDiskUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage ABI::Windows::System::Diagnostics::IProcessDiskUsage

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessDiskUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport ABI::Windows::System::Diagnostics::IProcessDiskUsageReport

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessMemoryUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage ABI::Windows::System::Diagnostics::IProcessMemoryUsage

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface IProcessMemoryUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport ABI::Windows::System::Diagnostics::IProcessMemoryUsageReport

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface ISystemCpuUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage ABI::Windows::System::Diagnostics::ISystemCpuUsage

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface ISystemCpuUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport ABI::Windows::System::Diagnostics::ISystemCpuUsageReport

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface ISystemDiagnosticInfo;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo ABI::Windows::System::Diagnostics::ISystemDiagnosticInfo

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface ISystemDiagnosticInfoStatics;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics ABI::Windows::System::Diagnostics::ISystemDiagnosticInfoStatics

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface ISystemDiagnosticInfoStatics2;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2 ABI::Windows::System::Diagnostics::ISystemDiagnosticInfoStatics2

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface ISystemMemoryUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage ABI::Windows::System::Diagnostics::ISystemMemoryUsage

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                interface ISystemMemoryUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport ABI::Windows::System::Diagnostics::ISystemMemoryUsageReport

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class DiagnosticActionResult;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                typedef enum DiagnosticActionState : int DiagnosticActionState;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("390b0091-caf7-5b64-839d-4990ae7f753c"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, enum ABI::Windows::System::Diagnostics::DiagnosticActionState> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, ABI::Windows::System::Diagnostics::IDiagnosticActionResult*>, enum ABI::Windows::System::Diagnostics::DiagnosticActionState>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.System.Diagnostics.DiagnosticActionResult, Windows.System.Diagnostics.DiagnosticActionState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, enum ABI::Windows::System::Diagnostics::DiagnosticActionState> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bb5d493e-74e9-57a1-8c4c-923e0dc4565b"))
IAsyncOperationWithProgress<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, enum ABI::Windows::System::Diagnostics::DiagnosticActionState> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, ABI::Windows::System::Diagnostics::IDiagnosticActionResult*>, enum ABI::Windows::System::Diagnostics::DiagnosticActionState>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.System.Diagnostics.DiagnosticActionResult, Windows.System.Diagnostics.DiagnosticActionState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, enum ABI::Windows::System::Diagnostics::DiagnosticActionState> __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_t;
#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a0422898-b50a-52e3-b461-53989308be12"))
IAsyncOperationProgressHandler<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, enum ABI::Windows::System::Diagnostics::DiagnosticActionState> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, ABI::Windows::System::Diagnostics::IDiagnosticActionResult*>, enum ABI::Windows::System::Diagnostics::DiagnosticActionState>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.System.Diagnostics.DiagnosticActionResult, Windows.System.Diagnostics.DiagnosticActionState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::System::Diagnostics::DiagnosticActionResult*, enum ABI::Windows::System::Diagnostics::DiagnosticActionState> __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace System {
            class AppDiagnosticInfo;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIAppDiagnosticInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIAppDiagnosticInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IAppDiagnosticInfo;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo ABI::Windows::System::IAppDiagnosticInfo

#endif // ____x_ABI_CWindows_CSystem_CIAppDiagnosticInfo_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_USE
#define DEF___FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("183f1e4a-2224-5fe4-b064-68869c53e361"))
IIterator<ABI::Windows::System::AppDiagnosticInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::AppDiagnosticInfo*, ABI::Windows::System::IAppDiagnosticInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.AppDiagnosticInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::AppDiagnosticInfo*> __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_t;
#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_USE
#define DEF___FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8118de8f-3ae3-55e1-80a8-25453cdba894"))
IIterable<ABI::Windows::System::AppDiagnosticInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::AppDiagnosticInfo*, ABI::Windows::System::IAppDiagnosticInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.AppDiagnosticInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::AppDiagnosticInfo*> __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_t;
#define __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class ProcessDiagnosticInfo;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE
#define DEF___FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a89b4418-4c3b-5f49-b957-785697c99abf"))
IIterator<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*, ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.Diagnostics.ProcessDiagnosticInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*> __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_t;
#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE
#define DEF___FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("97b73627-b296-5076-b8cd-6bd8a240e966"))
IIterable<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*, ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.Diagnostics.ProcessDiagnosticInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*> __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_t;
#define __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_USE
#define DEF___FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b0c2c7a4-78ba-50fd-84fe-00e02a6c1d42"))
IVectorView<ABI::Windows::System::AppDiagnosticInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::AppDiagnosticInfo*, ABI::Windows::System::IAppDiagnosticInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.System.AppDiagnosticInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::System::AppDiagnosticInfo*> __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_t;
#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE
#define DEF___FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("74ab2473-9624-5a06-9025-6d91e622bf8e"))
IVectorView<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*, ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.System.Diagnostics.ProcessDiagnosticInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::System::Diagnostics::ProcessDiagnosticInfo*> __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_t;
#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVector_1_Windows__CSystem__CAppDiagnosticInfo_USE
#define DEF___FIVector_1_Windows__CSystem__CAppDiagnosticInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9cffa2c3-7eeb-599c-b94d-c794b11f807f"))
IVector<ABI::Windows::System::AppDiagnosticInfo*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::AppDiagnosticInfo*, ABI::Windows::System::IAppDiagnosticInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.System.AppDiagnosticInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::System::AppDiagnosticInfo*> __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_t;
#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CSystem__CAppDiagnosticInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CSystem__CAppDiagnosticInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Json {
                class JsonObject;
            } /* Json */
        } /* Data */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CData_CJson_CIJsonObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CJson_CIJsonObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Json {
                interface IJsonObject;
            } /* Json */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CJson_CIJsonObject ABI::Windows::Data::Json::IJsonObject

#endif // ____x_ABI_CWindows_CData_CJson_CIJsonObject_FWD_DEFINED__

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
        namespace Foundation {
            typedef struct DateTime DateTime;
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
        namespace System {
            typedef enum ProcessorArchitecture : int ProcessorArchitecture;
        } /* System */
    } /* Windows */
} /* ABI */

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
        namespace System {
            namespace Diagnostics {
                class DiagnosticInvoker;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class ProcessCpuUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class ProcessCpuUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class ProcessDiskUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class ProcessDiskUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class ProcessMemoryUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class ProcessMemoryUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class SystemCpuUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class SystemCpuUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class SystemDiagnosticInfo;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class SystemMemoryUsage;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                class SystemMemoryUsageReport;
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Diagnostics.DiagnosticActionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                enum DiagnosticActionState : int
                {
                    DiagnosticActionState_Initializing = 0,
                    DiagnosticActionState_Downloading = 1,
                    DiagnosticActionState_VerifyingTrust = 2,
                    DiagnosticActionState_Detecting = 3,
                    DiagnosticActionState_Resolving = 4,
                    DiagnosticActionState_VerifyingResolution = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                    DiagnosticActionState_Executing = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                };
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IDiagnosticActionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DiagnosticActionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IDiagnosticActionResult[] = L"Windows.System.Diagnostics.IDiagnosticActionResult";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("c265a296-e73b-4097-b28f-3442f03dd831")
                IDiagnosticActionResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Results(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDiagnosticActionResult = __uuidof(IDiagnosticActionResult);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IDiagnosticInvoker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DiagnosticInvoker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IDiagnosticInvoker[] = L"Windows.System.Diagnostics.IDiagnosticInvoker";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("187b270a-02e3-4f86-84fc-fdd892b5940f")
                IDiagnosticInvoker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RunDiagnosticActionAsync(
                        ABI::Windows::Data::Json::IJsonObject* context,
                        __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDiagnosticInvoker = __uuidof(IDiagnosticInvoker);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IDiagnosticInvoker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DiagnosticInvoker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IDiagnosticInvoker2[] = L"Windows.System.Diagnostics.IDiagnosticInvoker2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("e3bf945c-155a-4b52-a8ec-070c44f95000")
                IDiagnosticInvoker2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RunDiagnosticActionFromStringAsync(
                        HSTRING context,
                        __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDiagnosticInvoker2 = __uuidof(IDiagnosticInvoker2);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Diagnostics.IDiagnosticInvokerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DiagnosticInvoker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IDiagnosticInvokerStatics[] = L"Windows.System.Diagnostics.IDiagnosticInvokerStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("5cfad8de-f15c-4554-a813-c113c3881b09")
                IDiagnosticInvokerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::System::Diagnostics::IDiagnosticInvoker** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::System::Diagnostics::IDiagnosticInvoker** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDiagnosticInvokerStatics = __uuidof(IDiagnosticInvokerStatics);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessCpuUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessCpuUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessCpuUsage[] = L"Windows.System.Diagnostics.IProcessCpuUsage";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("0bbb2472-c8bf-423a-a810-b559ae4354e2")
                IProcessCpuUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetReport(
                        ABI::Windows::System::Diagnostics::IProcessCpuUsageReport** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessCpuUsage = __uuidof(IProcessCpuUsage);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessCpuUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessCpuUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessCpuUsageReport[] = L"Windows.System.Diagnostics.IProcessCpuUsageReport";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("8a6d9cac-3987-4e2f-a119-6b5fa214f1b4")
                IProcessCpuUsageReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_KernelTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessCpuUsageReport = __uuidof(IProcessCpuUsageReport);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiagnosticInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiagnosticInfo[] = L"Windows.System.Diagnostics.IProcessDiagnosticInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("e830b04b-300e-4ee6-a0ab-5b5f5231b434")
                IProcessDiagnosticInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProcessId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExecutableFileName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Parent(
                        ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProcessStartTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DiskUsage(
                        ABI::Windows::System::Diagnostics::IProcessDiskUsage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MemoryUsage(
                        ABI::Windows::System::Diagnostics::IProcessMemoryUsage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CpuUsage(
                        ABI::Windows::System::Diagnostics::IProcessCpuUsage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessDiagnosticInfo = __uuidof(IProcessDiagnosticInfo);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiagnosticInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiagnosticInfo2[] = L"Windows.System.Diagnostics.IProcessDiagnosticInfo2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("9558cb1a-3d0b-49ec-ab70-4f7a112805de")
                IProcessDiagnosticInfo2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAppDiagnosticInfos(
                        __FIVector_1_Windows__CSystem__CAppDiagnosticInfo** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPackaged(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessDiagnosticInfo2 = __uuidof(IProcessDiagnosticInfo2);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiagnosticInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiagnosticInfoStatics[] = L"Windows.System.Diagnostics.IProcessDiagnosticInfoStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("2f41b260-b49f-428c-aa0e-84744f49ca95")
                IProcessDiagnosticInfoStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForProcesses(
                        __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo** processes
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentProcess(
                        ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo** processes
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessDiagnosticInfoStatics = __uuidof(IProcessDiagnosticInfoStatics);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiagnosticInfoStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiagnosticInfoStatics2[] = L"Windows.System.Diagnostics.IProcessDiagnosticInfoStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("4a869897-9899-4a44-a29b-091663be09b6")
                IProcessDiagnosticInfoStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryGetForProcessId(
                        UINT32 processId,
                        ABI::Windows::System::Diagnostics::IProcessDiagnosticInfo** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessDiagnosticInfoStatics2 = __uuidof(IProcessDiagnosticInfoStatics2);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiskUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiskUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiskUsage[] = L"Windows.System.Diagnostics.IProcessDiskUsage";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("5ad78bfd-7e51-4e53-bfaa-5a6ee1aabbf8")
                IProcessDiskUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetReport(
                        ABI::Windows::System::Diagnostics::IProcessDiskUsageReport** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessDiskUsage = __uuidof(IProcessDiskUsage);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiskUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiskUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiskUsageReport[] = L"Windows.System.Diagnostics.IProcessDiskUsageReport";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("401627fd-535d-4c1f-81b8-da54e1be635e")
                IProcessDiskUsageReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ReadOperationCount(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WriteOperationCount(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OtherOperationCount(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BytesReadCount(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BytesWrittenCount(
                        INT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OtherBytesCount(
                        INT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessDiskUsageReport = __uuidof(IProcessDiskUsageReport);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessMemoryUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessMemoryUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessMemoryUsage[] = L"Windows.System.Diagnostics.IProcessMemoryUsage";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("f50b229b-827c-42b7-b07c-0e32627e6b3e")
                IProcessMemoryUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetReport(
                        ABI::Windows::System::Diagnostics::IProcessMemoryUsageReport** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessMemoryUsage = __uuidof(IProcessMemoryUsage);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessMemoryUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessMemoryUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessMemoryUsageReport[] = L"Windows.System.Diagnostics.IProcessMemoryUsageReport";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("c2c77cba-1951-4685-8532-7e749ecf8eeb")
                IProcessMemoryUsageReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NonPagedPoolSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PageFaultCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PageFileSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PagedPoolSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PeakNonPagedPoolSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PeakPageFileSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PeakPagedPoolSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PeakVirtualMemorySizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PeakWorkingSetSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrivatePageCount(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VirtualMemorySizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WorkingSetSizeInBytes(
                        UINT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessMemoryUsageReport = __uuidof(IProcessMemoryUsageReport);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemCpuUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemCpuUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemCpuUsage[] = L"Windows.System.Diagnostics.ISystemCpuUsage";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("6037b3ac-02d6-4234-8362-7fe3adc81f5f")
                ISystemCpuUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetReport(
                        ABI::Windows::System::Diagnostics::ISystemCpuUsageReport** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemCpuUsage = __uuidof(ISystemCpuUsage);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemCpuUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemCpuUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemCpuUsageReport[] = L"Windows.System.Diagnostics.ISystemCpuUsageReport";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("2c26d0b2-9483-4f62-ab57-82b29d9719b8")
                ISystemCpuUsageReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_KernelTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IdleTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemCpuUsageReport = __uuidof(ISystemCpuUsageReport);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemDiagnosticInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemDiagnosticInfo[] = L"Windows.System.Diagnostics.ISystemDiagnosticInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("a290fe05-dff3-407f-9a1b-0b2b317ca800")
                ISystemDiagnosticInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MemoryUsage(
                        ABI::Windows::System::Diagnostics::ISystemMemoryUsage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CpuUsage(
                        ABI::Windows::System::Diagnostics::ISystemCpuUsage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemDiagnosticInfo = __uuidof(ISystemDiagnosticInfo);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemDiagnosticInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemDiagnosticInfoStatics[] = L"Windows.System.Diagnostics.ISystemDiagnosticInfoStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("d404ac21-fc7d-45f0-9a3f-39203aed9f7e")
                ISystemDiagnosticInfoStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentSystem(
                        ABI::Windows::System::Diagnostics::ISystemDiagnosticInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemDiagnosticInfoStatics = __uuidof(ISystemDiagnosticInfoStatics);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemDiagnosticInfoStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemDiagnosticInfoStatics2[] = L"Windows.System.Diagnostics.ISystemDiagnosticInfoStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("79ded189-6af9-4da9-a422-15f73255b3eb")
                ISystemDiagnosticInfoStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsArchitectureSupported(
                        ABI::Windows::System::ProcessorArchitecture type,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PreferredArchitecture(
                        ABI::Windows::System::ProcessorArchitecture* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemDiagnosticInfoStatics2 = __uuidof(ISystemDiagnosticInfoStatics2);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemMemoryUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemMemoryUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemMemoryUsage[] = L"Windows.System.Diagnostics.ISystemMemoryUsage";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("17ffc595-1702-49cf-aa27-2f0a32591404")
                ISystemMemoryUsage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetReport(
                        ABI::Windows::System::Diagnostics::ISystemMemoryUsageReport** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemMemoryUsage = __uuidof(ISystemMemoryUsage);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemMemoryUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemMemoryUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemMemoryUsageReport[] = L"Windows.System.Diagnostics.ISystemMemoryUsageReport";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                MIDL_INTERFACE("38663c87-2a9f-403a-bd19-2cf3e8169500")
                ISystemMemoryUsageReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TotalPhysicalSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AvailableSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CommittedSizeInBytes(
                        UINT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemMemoryUsageReport = __uuidof(ISystemMemoryUsageReport);
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.DiagnosticActionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IDiagnosticActionResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DiagnosticActionResult_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DiagnosticActionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DiagnosticActionResult[] = L"Windows.System.Diagnostics.DiagnosticActionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.System.Diagnostics.DiagnosticInvoker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.IDiagnosticInvokerStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IDiagnosticInvoker ** Default Interface **
 *    Windows.System.Diagnostics.IDiagnosticInvoker2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DiagnosticInvoker_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DiagnosticInvoker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DiagnosticInvoker[] = L"Windows.System.Diagnostics.DiagnosticInvoker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.System.Diagnostics.ProcessCpuUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessCpuUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessCpuUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessCpuUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessCpuUsage[] = L"Windows.System.Diagnostics.ProcessCpuUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessCpuUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessCpuUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessCpuUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessCpuUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessCpuUsageReport[] = L"Windows.System.Diagnostics.ProcessCpuUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.IProcessDiagnosticInfoStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.Diagnostics.IProcessDiagnosticInfoStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessDiagnosticInfo ** Default Interface **
 *    Windows.System.Diagnostics.IProcessDiagnosticInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiagnosticInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiagnosticInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessDiagnosticInfo[] = L"Windows.System.Diagnostics.ProcessDiagnosticInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessDiskUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessDiskUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiskUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiskUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessDiskUsage[] = L"Windows.System.Diagnostics.ProcessDiskUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessDiskUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessDiskUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiskUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiskUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessDiskUsageReport[] = L"Windows.System.Diagnostics.ProcessDiskUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessMemoryUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessMemoryUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessMemoryUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessMemoryUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessMemoryUsage[] = L"Windows.System.Diagnostics.ProcessMemoryUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessMemoryUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessMemoryUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessMemoryUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessMemoryUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessMemoryUsageReport[] = L"Windows.System.Diagnostics.ProcessMemoryUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.SystemCpuUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemCpuUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemCpuUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemCpuUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemCpuUsage[] = L"Windows.System.Diagnostics.SystemCpuUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.SystemCpuUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemCpuUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemCpuUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemCpuUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemCpuUsageReport[] = L"Windows.System.Diagnostics.SystemCpuUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.SystemDiagnosticInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.ISystemDiagnosticInfoStatics2 interface starting with version 11.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.Diagnostics.ISystemDiagnosticInfoStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemDiagnosticInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemDiagnosticInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemDiagnosticInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemDiagnosticInfo[] = L"Windows.System.Diagnostics.SystemDiagnosticInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.SystemMemoryUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemMemoryUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemMemoryUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemMemoryUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemMemoryUsage[] = L"Windows.System.Diagnostics.SystemMemoryUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.SystemMemoryUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemMemoryUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemMemoryUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemMemoryUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemMemoryUsageReport[] = L"Windows.System.Diagnostics.SystemMemoryUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2 __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2 __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2 __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2 __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CDiagnosticActionState __x_ABI_CWindows_CSystem_CDiagnostics_CDiagnosticActionState;

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* This,
        __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState* asyncInfo,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CDiagnosticActionState progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef ____x_ABI_CWindows_CSystem_CIAppDiagnosticInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIAppDiagnosticInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo;

#endif // ____x_ABI_CWindows_CSystem_CIAppDiagnosticInfo_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CAppDiagnosticInfo;

typedef struct __FIIterator_1_Windows__CSystem__CAppDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CAppDiagnosticInfoVtbl;

interface __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CAppDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CAppDiagnosticInfo;

typedef struct __FIIterable_1_Windows__CSystem__CAppDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CAppDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CAppDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CAppDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CAppDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CAppDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CAppDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CAppDiagnosticInfo* This,
        __FIIterator_1_Windows__CSystem__CAppDiagnosticInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CAppDiagnosticInfoVtbl;

interface __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CAppDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo;

typedef struct __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl;

interface __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo;

typedef struct __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        __FIIterator_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl;

interface __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo;

typedef struct __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfoVtbl;

interface __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo;

typedef struct __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl;

interface __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVector_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CSystem__CAppDiagnosticInfo __FIVector_1_Windows__CSystem__CAppDiagnosticInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CSystem__CAppDiagnosticInfo;

typedef struct __FIVector_1_Windows__CSystem__CAppDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        __FIVectorView_1_Windows__CSystem__CAppDiagnosticInfo** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CSystem__CAppDiagnosticInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CIAppDiagnosticInfo** items);

    END_INTERFACE
} __FIVector_1_Windows__CSystem__CAppDiagnosticInfoVtbl;

interface __FIVector_1_Windows__CSystem__CAppDiagnosticInfo
{
    CONST_VTBL struct __FIVector_1_Windows__CSystem__CAppDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CSystem__CAppDiagnosticInfo_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CSystem__CAppDiagnosticInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CData_CJson_CIJsonObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CJson_CIJsonObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CJson_CIJsonObject __x_ABI_CWindows_CData_CJson_CIJsonObject;

#endif // ____x_ABI_CWindows_CData_CJson_CIJsonObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CSystem_CProcessorArchitecture __x_ABI_CWindows_CSystem_CProcessorArchitecture;

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

/*
 *
 * Struct Windows.System.Diagnostics.DiagnosticActionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CDiagnosticActionState
{
    DiagnosticActionState_Initializing = 0,
    DiagnosticActionState_Downloading = 1,
    DiagnosticActionState_VerifyingTrust = 2,
    DiagnosticActionState_Detecting = 3,
    DiagnosticActionState_Resolving = 4,
    DiagnosticActionState_VerifyingResolution = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DiagnosticActionState_Executing = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IDiagnosticActionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DiagnosticActionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IDiagnosticActionResult[] = L"Windows.System.Diagnostics.IDiagnosticActionResult";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_Results)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResultVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_get_Results(This, value) \
    ((This)->lpVtbl->get_Results(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticActionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IDiagnosticInvoker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DiagnosticInvoker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IDiagnosticInvoker[] = L"Windows.System.Diagnostics.IDiagnosticInvoker";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RunDiagnosticActionAsync)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker* This,
        __x_ABI_CWindows_CData_CJson_CIJsonObject* context,
        __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_RunDiagnosticActionAsync(This, context, operation) \
    ((This)->lpVtbl->RunDiagnosticActionAsync(This, context, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IDiagnosticInvoker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DiagnosticInvoker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IDiagnosticInvoker2[] = L"Windows.System.Diagnostics.IDiagnosticInvoker2";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RunDiagnosticActionFromStringAsync)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2* This,
        HSTRING context,
        __FIAsyncOperationWithProgress_2_Windows__CSystem__CDiagnostics__CDiagnosticActionResult_Windows__CSystem__CDiagnostics__CDiagnosticActionState** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2Vtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_RunDiagnosticActionFromStringAsync(This, context, operation) \
    ((This)->lpVtbl->RunDiagnosticActionFromStringAsync(This, context, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Diagnostics.IDiagnosticInvokerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.DiagnosticInvoker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IDiagnosticInvokerStatics[] = L"Windows.System.Diagnostics.IDiagnosticInvokerStatics";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker** result);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvoker** result);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIDiagnosticInvokerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessCpuUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessCpuUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessCpuUsage[] = L"Windows.System.Diagnostics.IProcessCpuUsage";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetReport)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_GetReport(This, value) \
    ((This)->lpVtbl->GetReport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessCpuUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessCpuUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessCpuUsageReport[] = L"Windows.System.Diagnostics.IProcessCpuUsageReport";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KernelTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_UserTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReportVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_get_KernelTime(This, value) \
    ((This)->lpVtbl->get_KernelTime(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_get_UserTime(This, value) \
    ((This)->lpVtbl->get_UserTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiagnosticInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiagnosticInfo[] = L"Windows.System.Diagnostics.IProcessDiagnosticInfo";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProcessId)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ExecutableFileName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Parent)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ProcessStartTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_DiskUsage)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage** value);
    HRESULT (STDMETHODCALLTYPE* get_MemoryUsage)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage** value);
    HRESULT (STDMETHODCALLTYPE* get_CpuUsage)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessCpuUsage** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_get_ProcessId(This, value) \
    ((This)->lpVtbl->get_ProcessId(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_get_ExecutableFileName(This, value) \
    ((This)->lpVtbl->get_ExecutableFileName(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_get_Parent(This, value) \
    ((This)->lpVtbl->get_Parent(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_get_ProcessStartTime(This, value) \
    ((This)->lpVtbl->get_ProcessStartTime(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_get_DiskUsage(This, value) \
    ((This)->lpVtbl->get_DiskUsage(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_get_MemoryUsage(This, value) \
    ((This)->lpVtbl->get_MemoryUsage(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_get_CpuUsage(This, value) \
    ((This)->lpVtbl->get_CpuUsage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiagnosticInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiagnosticInfo2[] = L"Windows.System.Diagnostics.IProcessDiagnosticInfo2";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAppDiagnosticInfos)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2* This,
        __FIVector_1_Windows__CSystem__CAppDiagnosticInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_IsPackaged)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2Vtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_GetAppDiagnosticInfos(This, result) \
    ((This)->lpVtbl->GetAppDiagnosticInfos(This, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_get_IsPackaged(This, value) \
    ((This)->lpVtbl->get_IsPackaged(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiagnosticInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiagnosticInfoStatics[] = L"Windows.System.Diagnostics.IProcessDiagnosticInfoStatics";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForProcesses)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics* This,
        __FIVectorView_1_Windows__CSystem__CDiagnostics__CProcessDiagnosticInfo** processes);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentProcess)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo** processes);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_GetForProcesses(This, processes) \
    ((This)->lpVtbl->GetForProcesses(This, processes))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_GetForCurrentProcess(This, processes) \
    ((This)->lpVtbl->GetForCurrentProcess(This, processes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiagnosticInfoStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiagnosticInfoStatics2[] = L"Windows.System.Diagnostics.IProcessDiagnosticInfoStatics2";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetForProcessId)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2* This,
        UINT32 processId,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_TryGetForProcessId(This, processId, result) \
    ((This)->lpVtbl->TryGetForProcessId(This, processId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiagnosticInfoStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiskUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiskUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiskUsage[] = L"Windows.System.Diagnostics.IProcessDiskUsage";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetReport)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_GetReport(This, value) \
    ((This)->lpVtbl->GetReport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessDiskUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessDiskUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessDiskUsageReport[] = L"Windows.System.Diagnostics.IProcessDiskUsageReport";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ReadOperationCount)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_WriteOperationCount)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_OtherOperationCount)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_BytesReadCount)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_BytesWrittenCount)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        INT64* value);
    HRESULT (STDMETHODCALLTYPE* get_OtherBytesCount)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport* This,
        INT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReportVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_get_ReadOperationCount(This, value) \
    ((This)->lpVtbl->get_ReadOperationCount(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_get_WriteOperationCount(This, value) \
    ((This)->lpVtbl->get_WriteOperationCount(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_get_OtherOperationCount(This, value) \
    ((This)->lpVtbl->get_OtherOperationCount(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_get_BytesReadCount(This, value) \
    ((This)->lpVtbl->get_BytesReadCount(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_get_BytesWrittenCount(This, value) \
    ((This)->lpVtbl->get_BytesWrittenCount(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_get_OtherBytesCount(This, value) \
    ((This)->lpVtbl->get_OtherBytesCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessDiskUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessMemoryUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessMemoryUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessMemoryUsage[] = L"Windows.System.Diagnostics.IProcessMemoryUsage";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetReport)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_GetReport(This, value) \
    ((This)->lpVtbl->GetReport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.IProcessMemoryUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.ProcessMemoryUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_IProcessMemoryUsageReport[] = L"Windows.System.Diagnostics.IProcessMemoryUsageReport";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NonPagedPoolSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PageFaultCount)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PageFileSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PagedPoolSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PeakNonPagedPoolSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PeakPageFileSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PeakPagedPoolSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PeakVirtualMemorySizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PeakWorkingSetSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PrivatePageCount)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_VirtualMemorySizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_WorkingSetSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReportVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_NonPagedPoolSizeInBytes(This, value) \
    ((This)->lpVtbl->get_NonPagedPoolSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PageFaultCount(This, value) \
    ((This)->lpVtbl->get_PageFaultCount(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PageFileSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PageFileSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PagedPoolSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PagedPoolSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PeakNonPagedPoolSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PeakNonPagedPoolSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PeakPageFileSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PeakPageFileSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PeakPagedPoolSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PeakPagedPoolSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PeakVirtualMemorySizeInBytes(This, value) \
    ((This)->lpVtbl->get_PeakVirtualMemorySizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PeakWorkingSetSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PeakWorkingSetSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_PrivatePageCount(This, value) \
    ((This)->lpVtbl->get_PrivatePageCount(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_VirtualMemorySizeInBytes(This, value) \
    ((This)->lpVtbl->get_VirtualMemorySizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_get_WorkingSetSizeInBytes(This, value) \
    ((This)->lpVtbl->get_WorkingSetSizeInBytes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CIProcessMemoryUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemCpuUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemCpuUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemCpuUsage[] = L"Windows.System.Diagnostics.ISystemCpuUsage";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetReport)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_GetReport(This, value) \
    ((This)->lpVtbl->GetReport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemCpuUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemCpuUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemCpuUsageReport[] = L"Windows.System.Diagnostics.ISystemCpuUsageReport";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KernelTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_UserTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_IdleTime)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReportVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_get_KernelTime(This, value) \
    ((This)->lpVtbl->get_KernelTime(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_get_UserTime(This, value) \
    ((This)->lpVtbl->get_UserTime(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_get_IdleTime(This, value) \
    ((This)->lpVtbl->get_IdleTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemDiagnosticInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemDiagnosticInfo[] = L"Windows.System.Diagnostics.ISystemDiagnosticInfo";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MemoryUsage)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage** value);
    HRESULT (STDMETHODCALLTYPE* get_CpuUsage)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CISystemCpuUsage** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_get_MemoryUsage(This, value) \
    ((This)->lpVtbl->get_MemoryUsage(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_get_CpuUsage(This, value) \
    ((This)->lpVtbl->get_CpuUsage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemDiagnosticInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemDiagnosticInfoStatics[] = L"Windows.System.Diagnostics.ISystemDiagnosticInfoStatics";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentSystem)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_GetForCurrentSystem(This, value) \
    ((This)->lpVtbl->GetForCurrentSystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemDiagnosticInfoStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemDiagnosticInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemDiagnosticInfoStatics2[] = L"Windows.System.Diagnostics.ISystemDiagnosticInfoStatics2";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsArchitectureSupported)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2* This,
        enum __x_ABI_CWindows_CSystem_CProcessorArchitecture type,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_PreferredArchitecture)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2* This,
        enum __x_ABI_CWindows_CSystem_CProcessorArchitecture* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_IsArchitectureSupported(This, type, result) \
    ((This)->lpVtbl->IsArchitectureSupported(This, type, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_get_PreferredArchitecture(This, value) \
    ((This)->lpVtbl->get_PreferredArchitecture(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemDiagnosticInfoStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemMemoryUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemMemoryUsage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemMemoryUsage[] = L"Windows.System.Diagnostics.ISystemMemoryUsage";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetReport)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage* This,
        __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_GetReport(This, value) \
    ((This)->lpVtbl->GetReport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.ISystemMemoryUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.SystemMemoryUsageReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_ISystemMemoryUsageReport[] = L"Windows.System.Diagnostics.ISystemMemoryUsageReport";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TotalPhysicalSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_AvailableSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_CommittedSizeInBytes)(__x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReportVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_get_TotalPhysicalSizeInBytes(This, value) \
    ((This)->lpVtbl->get_TotalPhysicalSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_get_AvailableSizeInBytes(This, value) \
    ((This)->lpVtbl->get_AvailableSizeInBytes(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_get_CommittedSizeInBytes(This, value) \
    ((This)->lpVtbl->get_CommittedSizeInBytes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CISystemMemoryUsageReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.DiagnosticActionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IDiagnosticActionResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DiagnosticActionResult_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DiagnosticActionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DiagnosticActionResult[] = L"Windows.System.Diagnostics.DiagnosticActionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.System.Diagnostics.DiagnosticInvoker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.IDiagnosticInvokerStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IDiagnosticInvoker ** Default Interface **
 *    Windows.System.Diagnostics.IDiagnosticInvoker2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_DiagnosticInvoker_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_DiagnosticInvoker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_DiagnosticInvoker[] = L"Windows.System.Diagnostics.DiagnosticInvoker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.System.Diagnostics.ProcessCpuUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessCpuUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessCpuUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessCpuUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessCpuUsage[] = L"Windows.System.Diagnostics.ProcessCpuUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessCpuUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessCpuUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessCpuUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessCpuUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessCpuUsageReport[] = L"Windows.System.Diagnostics.ProcessCpuUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessDiagnosticInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.IProcessDiagnosticInfoStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.Diagnostics.IProcessDiagnosticInfoStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessDiagnosticInfo ** Default Interface **
 *    Windows.System.Diagnostics.IProcessDiagnosticInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiagnosticInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiagnosticInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessDiagnosticInfo[] = L"Windows.System.Diagnostics.ProcessDiagnosticInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessDiskUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessDiskUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiskUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiskUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessDiskUsage[] = L"Windows.System.Diagnostics.ProcessDiskUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessDiskUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessDiskUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiskUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessDiskUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessDiskUsageReport[] = L"Windows.System.Diagnostics.ProcessDiskUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessMemoryUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessMemoryUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessMemoryUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessMemoryUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessMemoryUsage[] = L"Windows.System.Diagnostics.ProcessMemoryUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.ProcessMemoryUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.IProcessMemoryUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_ProcessMemoryUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_ProcessMemoryUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_ProcessMemoryUsageReport[] = L"Windows.System.Diagnostics.ProcessMemoryUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Diagnostics.SystemCpuUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemCpuUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemCpuUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemCpuUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemCpuUsage[] = L"Windows.System.Diagnostics.SystemCpuUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.SystemCpuUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemCpuUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemCpuUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemCpuUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemCpuUsageReport[] = L"Windows.System.Diagnostics.SystemCpuUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.SystemDiagnosticInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.ISystemDiagnosticInfoStatics2 interface starting with version 11.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.Diagnostics.ISystemDiagnosticInfoStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemDiagnosticInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemDiagnosticInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemDiagnosticInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemDiagnosticInfo[] = L"Windows.System.Diagnostics.SystemDiagnosticInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.SystemMemoryUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemMemoryUsage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemMemoryUsage_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemMemoryUsage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemMemoryUsage[] = L"Windows.System.Diagnostics.SystemMemoryUsage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.SystemMemoryUsageReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.ISystemMemoryUsageReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_SystemMemoryUsageReport_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_SystemMemoryUsageReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_SystemMemoryUsageReport[] = L"Windows.System.Diagnostics.SystemMemoryUsageReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Ediagnostics_p_h__

#endif // __windows2Esystem2Ediagnostics_h__
