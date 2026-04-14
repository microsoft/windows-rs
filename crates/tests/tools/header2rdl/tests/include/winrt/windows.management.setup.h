
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
#ifndef __windows2Emanagement2Esetup_h__
#define __windows2Emanagement2Esetup_h__
#ifndef __windows2Emanagement2Esetup_p_h__
#define __windows2Emanagement2Esetup_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDeploymentSessionHeartbeatRequested;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested ABI::Windows::Management::Setup::IDeploymentSessionHeartbeatRequested

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IAgentProvisioningProgressReport;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport ABI::Windows::Management::Setup::IAgentProvisioningProgressReport

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDeploymentSessionConnectionChangedEventArgs;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs ABI::Windows::Management::Setup::IDeploymentSessionConnectionChangedEventArgs

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDeploymentSessionHeartbeatRequestedEventArgs;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs ABI::Windows::Management::Setup::IDeploymentSessionHeartbeatRequestedEventArgs

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDeploymentSessionStateChangedEventArgs;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs ABI::Windows::Management::Setup::IDeploymentSessionStateChangedEventArgs

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDeploymentWorkload;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload ABI::Windows::Management::Setup::IDeploymentWorkload

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDeploymentWorkloadBatch;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch ABI::Windows::Management::Setup::IDeploymentWorkloadBatch

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDeploymentWorkloadBatchFactory;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory ABI::Windows::Management::Setup::IDeploymentWorkloadBatchFactory

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDeploymentWorkloadFactory;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory ABI::Windows::Management::Setup::IDeploymentWorkloadFactory

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IDevicePreparationExecutionContext;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext ABI::Windows::Management::Setup::IDevicePreparationExecutionContext

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IMachineProvisioningProgressReporter;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter ABI::Windows::Management::Setup::IMachineProvisioningProgressReporter

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                interface IMachineProvisioningProgressReporterStatics;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics ABI::Windows::Management::Setup::IMachineProvisioningProgressReporterStatics

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                class DevicePreparationExecutionContext;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_USE
#define DEF___FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7275b4fe-ba33-55bd-830f-470f0f6d9a78"))
IAsyncOperation<ABI::Windows::Management::Setup::DevicePreparationExecutionContext*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DevicePreparationExecutionContext*, ABI::Windows::Management::Setup::IDevicePreparationExecutionContext*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Management.Setup.DevicePreparationExecutionContext>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Management::Setup::DevicePreparationExecutionContext*> __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_t;
#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c28c0ea5-d507-55d2-867e-a92213e563be"))
IAsyncOperationCompletedHandler<ABI::Windows::Management::Setup::DevicePreparationExecutionContext*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DevicePreparationExecutionContext*, ABI::Windows::Management::Setup::IDevicePreparationExecutionContext*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Management.Setup.DevicePreparationExecutionContext>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Management::Setup::DevicePreparationExecutionContext*> __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                class DeploymentWorkload;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE
#define DEF___FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bd27bfca-2f1c-5ffb-957a-2a7106c92e59"))
IIterator<ABI::Windows::Management::Setup::DeploymentWorkload*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentWorkload*, ABI::Windows::Management::Setup::IDeploymentWorkload*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Management.Setup.DeploymentWorkload>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Management::Setup::DeploymentWorkload*> __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_t;
#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE
#define DEF___FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0d04004b-d236-5ddc-a504-8df6b127e2bb"))
IIterable<ABI::Windows::Management::Setup::DeploymentWorkload*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentWorkload*, ABI::Windows::Management::Setup::IDeploymentWorkload*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Management.Setup.DeploymentWorkload>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Management::Setup::DeploymentWorkload*> __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_t;
#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                class DeploymentWorkloadBatch;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE
#define DEF___FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("db39f586-844a-5f5a-b73d-104956648b3e"))
IIterator<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*, ABI::Windows::Management::Setup::IDeploymentWorkloadBatch*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Management.Setup.DeploymentWorkloadBatch>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*> __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_t;
#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE
#define DEF___FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("88795c64-7f00-5ad8-86a0-76df57771cfd"))
IIterable<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*, ABI::Windows::Management::Setup::IDeploymentWorkloadBatch*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Management.Setup.DeploymentWorkloadBatch>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*> __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_t;
#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE
#define DEF___FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("55013416-d0b5-544e-b16d-95fcdb395f76"))
IVectorView<ABI::Windows::Management::Setup::DeploymentWorkload*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentWorkload*, ABI::Windows::Management::Setup::IDeploymentWorkload*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Management.Setup.DeploymentWorkload>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Management::Setup::DeploymentWorkload*> __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_t;
#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE
#define DEF___FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d5be9795-a43c-597e-b020-62ee328d2b56"))
IVectorView<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*, ABI::Windows::Management::Setup::IDeploymentWorkloadBatch*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Management.Setup.DeploymentWorkloadBatch>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*> __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_t;
#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE
#define DEF___FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("61b531b2-c040-5f95-9423-7fe1aab69d8d"))
IVector<ABI::Windows::Management::Setup::DeploymentWorkload*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentWorkload*, ABI::Windows::Management::Setup::IDeploymentWorkload*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Management.Setup.DeploymentWorkload>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Management::Setup::DeploymentWorkload*> __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_t;
#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE
#define DEF___FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0b1bd2b8-bbed-5d64-8712-ee309a3bfad9"))
IVector<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*, ABI::Windows::Management::Setup::IDeploymentWorkloadBatch*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Management.Setup.DeploymentWorkloadBatch>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Management::Setup::DeploymentWorkloadBatch*> __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_t;
#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

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
        namespace Management {
            namespace Setup {
                class MachineProvisioningProgressReporter;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                class DeploymentSessionConnectionChangedEventArgs;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3091fe0b-29c3-596c-bae3-aca1aed1a4dc"))
ITypedEventHandler<ABI::Windows::Management::Setup::MachineProvisioningProgressReporter*, ABI::Windows::Management::Setup::DeploymentSessionConnectionChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::MachineProvisioningProgressReporter*, ABI::Windows::Management::Setup::IMachineProvisioningProgressReporter*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentSessionConnectionChangedEventArgs*, ABI::Windows::Management::Setup::IDeploymentSessionConnectionChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Management.Setup.MachineProvisioningProgressReporter, Windows.Management.Setup.DeploymentSessionConnectionChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Management::Setup::MachineProvisioningProgressReporter*, ABI::Windows::Management::Setup::DeploymentSessionConnectionChangedEventArgs*> __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                class DeploymentSessionStateChangedEventArgs;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b0ab7b86-bb33-5a2c-9417-61dc56e65665"))
ITypedEventHandler<ABI::Windows::Management::Setup::MachineProvisioningProgressReporter*, ABI::Windows::Management::Setup::DeploymentSessionStateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::MachineProvisioningProgressReporter*, ABI::Windows::Management::Setup::IMachineProvisioningProgressReporter*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Setup::DeploymentSessionStateChangedEventArgs*, ABI::Windows::Management::Setup::IDeploymentSessionStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Management.Setup.MachineProvisioningProgressReporter, Windows.Management.Setup.DeploymentSessionStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Management::Setup::MachineProvisioningProgressReporter*, ABI::Windows::Management::Setup::DeploymentSessionStateChangedEventArgs*> __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

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

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                typedef enum DeploymentAgentProgressState : int DeploymentAgentProgressState;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                typedef enum DeploymentSessionConnectionChange : int DeploymentSessionConnectionChange;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                typedef enum DeploymentSessionStateChange : int DeploymentSessionStateChange;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                typedef enum DeploymentWorkloadState : int DeploymentWorkloadState;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                class AgentProvisioningProgressReport;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                class DeploymentSessionHeartbeatRequestedEventArgs;
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Management.Setup.DeploymentAgentProgressState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                enum DeploymentAgentProgressState : int
                {
                    DeploymentAgentProgressState_NotStarted = 0,
                    DeploymentAgentProgressState_Initializing = 1,
                    DeploymentAgentProgressState_InProgress = 2,
                    DeploymentAgentProgressState_Completed = 3,
                    DeploymentAgentProgressState_ErrorOccurred = 4,
                    DeploymentAgentProgressState_RebootRequired = 5,
                    DeploymentAgentProgressState_Canceled = 6,
                };
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.Management.Setup.DeploymentSessionConnectionChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                enum DeploymentSessionConnectionChange : int
                {
                    DeploymentSessionConnectionChange_NoChange = 0,
                    DeploymentSessionConnectionChange_HostConnectionLost = 1,
                    DeploymentSessionConnectionChange_HostConnectionRestored = 2,
                    DeploymentSessionConnectionChange_AgentConnectionLost = 3,
                    DeploymentSessionConnectionChange_AgentConnectionRestored = 4,
                    DeploymentSessionConnectionChange_InternetConnectionLost = 5,
                    DeploymentSessionConnectionChange_InternetConnectionRestored = 6,
                };
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.Management.Setup.DeploymentSessionStateChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                enum DeploymentSessionStateChange : int
                {
                    DeploymentSessionStateChange_NoChange = 0,
                    DeploymentSessionStateChange_CancelRequestedByUser = 1,
                    DeploymentSessionStateChange_RetryRequestedByUser = 2,
                };
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.Management.Setup.DeploymentWorkloadState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                enum DeploymentWorkloadState : int
                {
                    DeploymentWorkloadState_NotStarted = 0,
                    DeploymentWorkloadState_InProgress = 1,
                    DeploymentWorkloadState_Completed = 2,
                    DeploymentWorkloadState_Failed = 3,
                    DeploymentWorkloadState_Canceled = 4,
                    DeploymentWorkloadState_Skipped = 5,
                    DeploymentWorkloadState_Uninstalled = 6,
                    DeploymentWorkloadState_RebootRequired = 7,
                };
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Delegate Windows.Management.Setup.DeploymentSessionHeartbeatRequested
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("c94a770b-5b05-4595-9e69-79070484377e")
                IDeploymentSessionHeartbeatRequested : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Management::Setup::IDeploymentSessionHeartbeatRequestedEventArgs* eventArgs
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeploymentSessionHeartbeatRequested = __uuidof(IDeploymentSessionHeartbeatRequested);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IAgentProvisioningProgressReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.AgentProvisioningProgressReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IAgentProvisioningProgressReport[] = L"Windows.Management.Setup.IAgentProvisioningProgressReport";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("5097398a-70cc-5181-a7af-d31c167323d1")
                IAgentProvisioningProgressReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Management::Setup::DeploymentAgentProgressState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_State(
                        ABI::Windows::Management::Setup::DeploymentAgentProgressState value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProgressPercentage(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProgressPercentage(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EstimatedTimeRemaining(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EstimatedTimeRemaining(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayProgress(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayProgress(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayProgressSecondary(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayProgressSecondary(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Batches(
                        __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentBatchIndex(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CurrentBatchIndex(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAgentProvisioningProgressReport = __uuidof(IAgentProvisioningProgressReport);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentSessionConnectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentSessionConnectionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentSessionConnectionChangedEventArgs[] = L"Windows.Management.Setup.IDeploymentSessionConnectionChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("8d40c631-6e4b-5d59-92f8-0de54c2a3c6b")
                IDeploymentSessionConnectionChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Change(
                        ABI::Windows::Management::Setup::DeploymentSessionConnectionChange* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeploymentSessionConnectionChangedEventArgs = __uuidof(IDeploymentSessionConnectionChangedEventArgs);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentSessionHeartbeatRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentSessionHeartbeatRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentSessionHeartbeatRequestedEventArgs[] = L"Windows.Management.Setup.IDeploymentSessionHeartbeatRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("09d81fa0-1036-58e6-b63b-fe343c45005f")
                IDeploymentSessionHeartbeatRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeploymentSessionHeartbeatRequestedEventArgs = __uuidof(IDeploymentSessionHeartbeatRequestedEventArgs);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentSessionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentSessionStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentSessionStateChangedEventArgs[] = L"Windows.Management.Setup.IDeploymentSessionStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("fbd3b7f3-88cb-5703-b8a5-0218de8fed81")
                IDeploymentSessionStateChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Change(
                        ABI::Windows::Management::Setup::DeploymentSessionStateChange* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeploymentSessionStateChangedEventArgs = __uuidof(IDeploymentSessionStateChangedEventArgs);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentWorkload
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentWorkload
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentWorkload[] = L"Windows.Management.Setup.IDeploymentWorkload";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("1cefd3d4-456c-50d1-9312-cc5c818fc12e")
                IDeploymentWorkload : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayFriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayFriendlyName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StartTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EndTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ErrorCode(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ErrorMessage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ErrorMessage(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PossibleCause(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PossibleCause(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PossibleResolution(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PossibleResolution(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Management::Setup::DeploymentWorkloadState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_State(
                        ABI::Windows::Management::Setup::DeploymentWorkloadState value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StateDetails(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StateDetails(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeploymentWorkload = __uuidof(IDeploymentWorkload);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentWorkloadBatch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentWorkloadBatch
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentWorkloadBatch[] = L"Windows.Management.Setup.IDeploymentWorkloadBatch";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("5e56e3df-b9c0-5fee-ba3f-e89d800a9bf2")
                IDeploymentWorkloadBatch : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayCategoryTitle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayCategoryTitle(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BatchWorkloads(
                        __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeploymentWorkloadBatch = __uuidof(IDeploymentWorkloadBatch);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentWorkloadBatchFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentWorkloadBatch
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentWorkloadBatchFactory[] = L"Windows.Management.Setup.IDeploymentWorkloadBatchFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("d0209697-9560-5a05-bdf6-f1af535cb0d4")
                IDeploymentWorkloadBatchFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        UINT32 id,
                        ABI::Windows::Management::Setup::IDeploymentWorkloadBatch** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeploymentWorkloadBatchFactory = __uuidof(IDeploymentWorkloadBatchFactory);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentWorkloadFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentWorkload
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentWorkloadFactory[] = L"Windows.Management.Setup.IDeploymentWorkloadFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("41426c72-22a3-5339-bdf1-51268169aa61")
                IDeploymentWorkloadFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING id,
                        ABI::Windows::Management::Setup::IDeploymentWorkload** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeploymentWorkloadFactory = __uuidof(IDeploymentWorkloadFactory);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDevicePreparationExecutionContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DevicePreparationExecutionContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDevicePreparationExecutionContext[] = L"Windows.Management.Setup.IDevicePreparationExecutionContext";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("084f221b-2484-5e81-a4e7-83f6caf19dc4")
                IDevicePreparationExecutionContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Context(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePreparationExecutionContext = __uuidof(IDevicePreparationExecutionContext);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IMachineProvisioningProgressReporter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.MachineProvisioningProgressReporter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IMachineProvisioningProgressReporter[] = L"Windows.Management.Setup.IMachineProvisioningProgressReporter";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("ebd8677f-dfd2-59da-ac3d-753ee1667cbb")
                IMachineProvisioningProgressReporter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SessionConnection(
                        ABI::Windows::Management::Setup::DeploymentSessionConnectionChange* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SessionState(
                        ABI::Windows::Management::Setup::DeploymentSessionStateChange* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SessionStateChanged(
                        __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SessionStateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SessionConnectionChanged(
                        __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SessionConnectionChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportProgress(
                        ABI::Windows::Management::Setup::IAgentProvisioningProgressReport* updateReport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDevicePreparationExecutionContextAsync(
                        __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMachineProvisioningProgressReporter = __uuidof(IMachineProvisioningProgressReporter);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IMachineProvisioningProgressReporterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.MachineProvisioningProgressReporter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IMachineProvisioningProgressReporterStatics[] = L"Windows.Management.Setup.IMachineProvisioningProgressReporterStatics";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Setup {
                MIDL_INTERFACE("77682c17-5da3-51fc-a042-c7b53458ddb5")
                IMachineProvisioningProgressReporterStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForLaunchUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* launchUri,
                        ABI::Windows::Management::Setup::IDeploymentSessionHeartbeatRequested* heartbeatHandler,
                        ABI::Windows::Management::Setup::IMachineProvisioningProgressReporter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMachineProvisioningProgressReporterStatics = __uuidof(IMachineProvisioningProgressReporterStatics);
            } /* Setup */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.AgentProvisioningProgressReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IAgentProvisioningProgressReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_AgentProvisioningProgressReport_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_AgentProvisioningProgressReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_AgentProvisioningProgressReport[] = L"Windows.Management.Setup.AgentProvisioningProgressReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentSessionConnectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentSessionConnectionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionConnectionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionConnectionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentSessionConnectionChangedEventArgs[] = L"Windows.Management.Setup.DeploymentSessionConnectionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentSessionHeartbeatRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentSessionHeartbeatRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionHeartbeatRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionHeartbeatRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentSessionHeartbeatRequestedEventArgs[] = L"Windows.Management.Setup.DeploymentSessionHeartbeatRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentSessionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentSessionStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentSessionStateChangedEventArgs[] = L"Windows.Management.Setup.DeploymentSessionStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentWorkload
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Setup.IDeploymentWorkloadFactory interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentWorkload ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentWorkload_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentWorkload_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentWorkload[] = L"Windows.Management.Setup.DeploymentWorkload";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentWorkloadBatch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Setup.IDeploymentWorkloadBatchFactory interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentWorkloadBatch ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentWorkloadBatch_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentWorkloadBatch_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentWorkloadBatch[] = L"Windows.Management.Setup.DeploymentWorkloadBatch";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DevicePreparationExecutionContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDevicePreparationExecutionContext ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DevicePreparationExecutionContext_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DevicePreparationExecutionContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DevicePreparationExecutionContext[] = L"Windows.Management.Setup.DevicePreparationExecutionContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.MachineProvisioningProgressReporter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Setup.IMachineProvisioningProgressReporterStatics interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IMachineProvisioningProgressReporter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_MachineProvisioningProgressReporter_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_MachineProvisioningProgressReporter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_MachineProvisioningProgressReporter[] = L"Windows.Management.Setup.MachineProvisioningProgressReporter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics;

#endif // ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext;

typedef struct __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContextVtbl;

interface __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* This,
        __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContextVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload;

typedef struct __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl;

interface __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload
{
    CONST_VTBL struct __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload;

typedef struct __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkload** result);

    END_INTERFACE
} __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl;

interface __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload
{
    CONST_VTBL struct __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch;

typedef struct __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl;

interface __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch
{
    CONST_VTBL struct __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch;

typedef struct __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        __FIIterator_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch** result);

    END_INTERFACE
} __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl;

interface __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch
{
    CONST_VTBL struct __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload;

typedef struct __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl;

interface __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload
{
    CONST_VTBL struct __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch;

typedef struct __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl;

interface __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch
{
    CONST_VTBL struct __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload;

typedef struct __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkload** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload** items);

    END_INTERFACE
} __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl;

interface __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload
{
    CONST_VTBL struct __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch;

typedef struct __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        __FIVectorView_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch** items);

    END_INTERFACE
} __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl;

interface __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch
{
    CONST_VTBL struct __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatchVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs* This,
        __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* sender,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs* This,
        __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* sender,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentAgentProgressState __x_ABI_CWindows_CManagement_CSetup_CDeploymentAgentProgressState;

typedef enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionConnectionChange __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionConnectionChange;

typedef enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionStateChange __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionStateChange;

typedef enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentWorkloadState __x_ABI_CWindows_CManagement_CSetup_CDeploymentWorkloadState;

/*
 *
 * Struct Windows.Management.Setup.DeploymentAgentProgressState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentAgentProgressState
{
    DeploymentAgentProgressState_NotStarted = 0,
    DeploymentAgentProgressState_Initializing = 1,
    DeploymentAgentProgressState_InProgress = 2,
    DeploymentAgentProgressState_Completed = 3,
    DeploymentAgentProgressState_ErrorOccurred = 4,
    DeploymentAgentProgressState_RebootRequired = 5,
    DeploymentAgentProgressState_Canceled = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.Management.Setup.DeploymentSessionConnectionChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionConnectionChange
{
    DeploymentSessionConnectionChange_NoChange = 0,
    DeploymentSessionConnectionChange_HostConnectionLost = 1,
    DeploymentSessionConnectionChange_HostConnectionRestored = 2,
    DeploymentSessionConnectionChange_AgentConnectionLost = 3,
    DeploymentSessionConnectionChange_AgentConnectionRestored = 4,
    DeploymentSessionConnectionChange_InternetConnectionLost = 5,
    DeploymentSessionConnectionChange_InternetConnectionRestored = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.Management.Setup.DeploymentSessionStateChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionStateChange
{
    DeploymentSessionStateChange_NoChange = 0,
    DeploymentSessionStateChange_CancelRequestedByUser = 1,
    DeploymentSessionStateChange_RetryRequestedByUser = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.Management.Setup.DeploymentWorkloadState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentWorkloadState
{
    DeploymentWorkloadState_NotStarted = 0,
    DeploymentWorkloadState_InProgress = 1,
    DeploymentWorkloadState_Completed = 2,
    DeploymentWorkloadState_Failed = 3,
    DeploymentWorkloadState_Canceled = 4,
    DeploymentWorkloadState_Skipped = 5,
    DeploymentWorkloadState_Uninstalled = 6,
    DeploymentWorkloadState_RebootRequired = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Delegate Windows.Management.Setup.DeploymentSessionHeartbeatRequested
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested* This,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* eventArgs);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_Invoke(This, eventArgs) \
    ((This)->lpVtbl->Invoke(This, eventArgs))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IAgentProvisioningProgressReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.AgentProvisioningProgressReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IAgentProvisioningProgressReport[] = L"Windows.Management.Setup.IAgentProvisioningProgressReport";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentAgentProgressState* value);
    HRESULT (STDMETHODCALLTYPE* put_State)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentAgentProgressState value);
    HRESULT (STDMETHODCALLTYPE* get_ProgressPercentage)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_ProgressPercentage)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_EstimatedTimeRemaining)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_EstimatedTimeRemaining)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayProgress)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayProgress)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayProgressSecondary)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayProgressSecondary)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Batches)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkloadBatch** value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentBatchIndex)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_CurrentBatchIndex)(__x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReportVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_put_State(This, value) \
    ((This)->lpVtbl->put_State(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_get_ProgressPercentage(This, value) \
    ((This)->lpVtbl->get_ProgressPercentage(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_put_ProgressPercentage(This, value) \
    ((This)->lpVtbl->put_ProgressPercentage(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_get_EstimatedTimeRemaining(This, value) \
    ((This)->lpVtbl->get_EstimatedTimeRemaining(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_put_EstimatedTimeRemaining(This, value) \
    ((This)->lpVtbl->put_EstimatedTimeRemaining(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_get_DisplayProgress(This, value) \
    ((This)->lpVtbl->get_DisplayProgress(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_put_DisplayProgress(This, value) \
    ((This)->lpVtbl->put_DisplayProgress(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_get_DisplayProgressSecondary(This, value) \
    ((This)->lpVtbl->get_DisplayProgressSecondary(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_put_DisplayProgressSecondary(This, value) \
    ((This)->lpVtbl->put_DisplayProgressSecondary(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_get_Batches(This, value) \
    ((This)->lpVtbl->get_Batches(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_get_CurrentBatchIndex(This, value) \
    ((This)->lpVtbl->get_CurrentBatchIndex(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_put_CurrentBatchIndex(This, value) \
    ((This)->lpVtbl->put_CurrentBatchIndex(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentSessionConnectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentSessionConnectionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentSessionConnectionChangedEventArgs[] = L"Windows.Management.Setup.IDeploymentSessionConnectionChangedEventArgs";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Change)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs* This,
        enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionConnectionChange* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgsVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_get_Change(This, value) \
    ((This)->lpVtbl->get_Change(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionConnectionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentSessionHeartbeatRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentSessionHeartbeatRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentSessionHeartbeatRequestedEventArgs[] = L"Windows.Management.Setup.IDeploymentSessionHeartbeatRequestedEventArgs";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentSessionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentSessionStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentSessionStateChangedEventArgs[] = L"Windows.Management.Setup.IDeploymentSessionStateChangedEventArgs";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Change)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs* This,
        enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionStateChange* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_get_Change(This, value) \
    ((This)->lpVtbl->get_Change(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentWorkload
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentWorkload
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentWorkload[] = L"Windows.Management.Setup.IDeploymentWorkload";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayFriendlyName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayFriendlyName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_StartTime)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_EndTime)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_EndTime)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ErrorCode)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorMessage)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ErrorMessage)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PossibleCause)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PossibleCause)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PossibleResolution)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PossibleResolution)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentWorkloadState* value);
    HRESULT (STDMETHODCALLTYPE* put_State)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentWorkloadState value);
    HRESULT (STDMETHODCALLTYPE* get_StateDetails)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_StateDetails)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_DisplayFriendlyName(This, value) \
    ((This)->lpVtbl->get_DisplayFriendlyName(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_DisplayFriendlyName(This, value) \
    ((This)->lpVtbl->put_DisplayFriendlyName(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_StartTime(This, value) \
    ((This)->lpVtbl->put_StartTime(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_EndTime(This, value) \
    ((This)->lpVtbl->get_EndTime(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_EndTime(This, value) \
    ((This)->lpVtbl->put_EndTime(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_ErrorCode(This, value) \
    ((This)->lpVtbl->put_ErrorCode(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_ErrorMessage(This, value) \
    ((This)->lpVtbl->get_ErrorMessage(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_ErrorMessage(This, value) \
    ((This)->lpVtbl->put_ErrorMessage(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_PossibleCause(This, value) \
    ((This)->lpVtbl->get_PossibleCause(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_PossibleCause(This, value) \
    ((This)->lpVtbl->put_PossibleCause(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_PossibleResolution(This, value) \
    ((This)->lpVtbl->get_PossibleResolution(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_PossibleResolution(This, value) \
    ((This)->lpVtbl->put_PossibleResolution(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_State(This, value) \
    ((This)->lpVtbl->put_State(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_get_StateDetails(This, value) \
    ((This)->lpVtbl->get_StateDetails(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_put_StateDetails(This, value) \
    ((This)->lpVtbl->put_StateDetails(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentWorkloadBatch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentWorkloadBatch
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentWorkloadBatch[] = L"Windows.Management.Setup.IDeploymentWorkloadBatch";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayCategoryTitle)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayCategoryTitle)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_BatchWorkloads)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch* This,
        __FIVector_1_Windows__CManagement__CSetup__CDeploymentWorkload** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_get_DisplayCategoryTitle(This, value) \
    ((This)->lpVtbl->get_DisplayCategoryTitle(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_put_DisplayCategoryTitle(This, value) \
    ((This)->lpVtbl->put_DisplayCategoryTitle(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_get_BatchWorkloads(This, value) \
    ((This)->lpVtbl->get_BatchWorkloads(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentWorkloadBatchFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentWorkloadBatch
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentWorkloadBatchFactory[] = L"Windows.Management.Setup.IDeploymentWorkloadBatchFactory";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory* This,
        UINT32 id,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatch** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_CreateInstance(This, id, value) \
    ((This)->lpVtbl->CreateInstance(This, id, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadBatchFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDeploymentWorkloadFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DeploymentWorkload
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDeploymentWorkloadFactory[] = L"Windows.Management.Setup.IDeploymentWorkloadFactory";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory* This,
        HSTRING id,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkload** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_CreateInstance(This, id, value) \
    ((This)->lpVtbl->CreateInstance(This, id, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDeploymentWorkloadFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IDevicePreparationExecutionContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.DevicePreparationExecutionContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IDevicePreparationExecutionContext[] = L"Windows.Management.Setup.IDevicePreparationExecutionContext";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Context)(__x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContextVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_get_Context(This, value) \
    ((This)->lpVtbl->get_Context(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIDevicePreparationExecutionContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IMachineProvisioningProgressReporter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.MachineProvisioningProgressReporter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IMachineProvisioningProgressReporter[] = L"Windows.Management.Setup.IMachineProvisioningProgressReporter";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_SessionConnection)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionConnectionChange* value);
    HRESULT (STDMETHODCALLTYPE* get_SessionState)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        enum __x_ABI_CWindows_CManagement_CSetup_CDeploymentSessionStateChange* value);
    HRESULT (STDMETHODCALLTYPE* add_SessionStateChanged)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionStateChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SessionStateChanged)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SessionConnectionChanged)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        __FITypedEventHandler_2_Windows__CManagement__CSetup__CMachineProvisioningProgressReporter_Windows__CManagement__CSetup__CDeploymentSessionConnectionChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SessionConnectionChanged)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* ReportProgress)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        __x_ABI_CWindows_CManagement_CSetup_CIAgentProvisioningProgressReport* updateReport);
    HRESULT (STDMETHODCALLTYPE* GetDevicePreparationExecutionContextAsync)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter* This,
        __FIAsyncOperation_1_Windows__CManagement__CSetup__CDevicePreparationExecutionContext** operation);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_get_SessionConnection(This, value) \
    ((This)->lpVtbl->get_SessionConnection(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_get_SessionState(This, value) \
    ((This)->lpVtbl->get_SessionState(This, value))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_add_SessionStateChanged(This, handler, token) \
    ((This)->lpVtbl->add_SessionStateChanged(This, handler, token))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_remove_SessionStateChanged(This, token) \
    ((This)->lpVtbl->remove_SessionStateChanged(This, token))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_add_SessionConnectionChanged(This, handler, token) \
    ((This)->lpVtbl->add_SessionConnectionChanged(This, handler, token))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_remove_SessionConnectionChanged(This, token) \
    ((This)->lpVtbl->remove_SessionConnectionChanged(This, token))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_ReportProgress(This, updateReport) \
    ((This)->lpVtbl->ReportProgress(This, updateReport))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_GetDevicePreparationExecutionContextAsync(This, operation) \
    ((This)->lpVtbl->GetDevicePreparationExecutionContextAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Management.Setup.IMachineProvisioningProgressReporterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Management.Setup.MachineProvisioningProgressReporter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Setup_IMachineProvisioningProgressReporterStatics[] = L"Windows.Management.Setup.IMachineProvisioningProgressReporterStatics";
typedef struct __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForLaunchUri)(__x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* launchUri,
        __x_ABI_CWindows_CManagement_CSetup_CIDeploymentSessionHeartbeatRequested* heartbeatHandler,
        __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporter** result);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStaticsVtbl;

interface __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_GetForLaunchUri(This, launchUri, heartbeatHandler, result) \
    ((This)->lpVtbl->GetForLaunchUri(This, launchUri, heartbeatHandler, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CSetup_CIMachineProvisioningProgressReporterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.AgentProvisioningProgressReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IAgentProvisioningProgressReport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_AgentProvisioningProgressReport_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_AgentProvisioningProgressReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_AgentProvisioningProgressReport[] = L"Windows.Management.Setup.AgentProvisioningProgressReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentSessionConnectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentSessionConnectionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionConnectionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionConnectionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentSessionConnectionChangedEventArgs[] = L"Windows.Management.Setup.DeploymentSessionConnectionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentSessionHeartbeatRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentSessionHeartbeatRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionHeartbeatRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionHeartbeatRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentSessionHeartbeatRequestedEventArgs[] = L"Windows.Management.Setup.DeploymentSessionHeartbeatRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentSessionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentSessionStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentSessionStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentSessionStateChangedEventArgs[] = L"Windows.Management.Setup.DeploymentSessionStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentWorkload
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Setup.IDeploymentWorkloadFactory interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentWorkload ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentWorkload_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentWorkload_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentWorkload[] = L"Windows.Management.Setup.DeploymentWorkload";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DeploymentWorkloadBatch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Setup.IDeploymentWorkloadBatchFactory interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDeploymentWorkloadBatch ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DeploymentWorkloadBatch_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DeploymentWorkloadBatch_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DeploymentWorkloadBatch[] = L"Windows.Management.Setup.DeploymentWorkloadBatch";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.DevicePreparationExecutionContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IDevicePreparationExecutionContext ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_DevicePreparationExecutionContext_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_DevicePreparationExecutionContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_DevicePreparationExecutionContext[] = L"Windows.Management.Setup.DevicePreparationExecutionContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Management.Setup.MachineProvisioningProgressReporter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Setup.IMachineProvisioningProgressReporterStatics interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Setup.IMachineProvisioningProgressReporter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Management_Setup_MachineProvisioningProgressReporter_DEFINED
#define RUNTIMECLASS_Windows_Management_Setup_MachineProvisioningProgressReporter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Setup_MachineProvisioningProgressReporter[] = L"Windows.Management.Setup.MachineProvisioningProgressReporter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emanagement2Esetup_p_h__

#endif // __windows2Emanagement2Esetup_h__
