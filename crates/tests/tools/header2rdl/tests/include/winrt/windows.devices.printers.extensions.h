
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
#ifndef __windows2Edevices2Eprinters2Eextensions_h__
#define __windows2Edevices2Eprinters2Eextensions_h__
#ifndef __windows2Edevices2Eprinters2Eextensions_p_h__
#define __windows2Edevices2Eprinters2Eextensions_p_h__


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
#if !defined(WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION)
#define WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION)

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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrint3DWorkflow;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflow

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrint3DWorkflow2;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2 ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflow2

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrint3DWorkflowPrintRequestedEventArgs;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflowPrintRequestedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrint3DWorkflowPrinterChangedEventArgs;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflowPrinterChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrintExtensionContextStatic;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic ABI::Windows::Devices::Printers::Extensions::IPrintExtensionContextStatic

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrintNotificationEventDetails;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails ABI::Windows::Devices::Printers::Extensions::IPrintNotificationEventDetails

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrintTaskConfiguration;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfiguration

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrintTaskConfigurationSaveRequest;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfigurationSaveRequest

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrintTaskConfigurationSaveRequestedDeferral;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfigurationSaveRequestedDeferral

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrintTaskConfigurationSaveRequestedEventArgs;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfigurationSaveRequestedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    class Print3DWorkflow;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    class Print3DWorkflowPrintRequestedEventArgs;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5f4c6603-5512-59aa-8d96-b1389d8b5796"))
ITypedEventHandler<ABI::Windows::Devices::Printers::Extensions::Print3DWorkflow*, ABI::Windows::Devices::Printers::Extensions::Print3DWorkflowPrintRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Printers::Extensions::Print3DWorkflow*, ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Printers::Extensions::Print3DWorkflowPrintRequestedEventArgs*, ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflowPrintRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Printers.Extensions.Print3DWorkflow, Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Printers::Extensions::Print3DWorkflow*, ABI::Windows::Devices::Printers::Extensions::Print3DWorkflowPrintRequestedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    class Print3DWorkflowPrinterChangedEventArgs;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b4b5ddc2-1a74-5905-9fc5-ddaae9a3ab93"))
ITypedEventHandler<ABI::Windows::Devices::Printers::Extensions::Print3DWorkflow*, ABI::Windows::Devices::Printers::Extensions::Print3DWorkflowPrinterChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Printers::Extensions::Print3DWorkflow*, ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Printers::Extensions::Print3DWorkflowPrinterChangedEventArgs*, ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflowPrinterChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Printers.Extensions.Print3DWorkflow, Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Printers::Extensions::Print3DWorkflow*, ABI::Windows::Devices::Printers::Extensions::Print3DWorkflowPrinterChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    class PrintTaskConfiguration;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    class PrintTaskConfigurationSaveRequestedEventArgs;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0363f57a-b7a2-5e20-a156-253423e7ee40"))
ITypedEventHandler<ABI::Windows::Devices::Printers::Extensions::PrintTaskConfiguration*, ABI::Windows::Devices::Printers::Extensions::PrintTaskConfigurationSaveRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Printers::Extensions::PrintTaskConfiguration*, ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfiguration*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Printers::Extensions::PrintTaskConfigurationSaveRequestedEventArgs*, ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfigurationSaveRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Printers.Extensions.PrintTaskConfiguration, Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Printers::Extensions::PrintTaskConfiguration*, ABI::Windows::Devices::Printers::Extensions::PrintTaskConfigurationSaveRequestedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_USE */

#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    typedef enum Print3DWorkflowDetail : int Print3DWorkflowDetail;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    typedef enum Print3DWorkflowStatus : int Print3DWorkflowStatus;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    class PrintTaskConfigurationSaveRequest;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    class PrintTaskConfigurationSaveRequestedDeferral;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Printers.Extensions.Print3DWorkflowDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    enum Print3DWorkflowDetail : int
                    {
                        Print3DWorkflowDetail_Unknown = 0,
                        Print3DWorkflowDetail_ModelExceedsPrintBed = 1,
                        Print3DWorkflowDetail_UploadFailed = 2,
                        Print3DWorkflowDetail_InvalidMaterialSelection = 3,
                        Print3DWorkflowDetail_InvalidModel = 4,
                        Print3DWorkflowDetail_ModelNotManifold = 5,
                        Print3DWorkflowDetail_InvalidPrintTicket = 6,
                    };
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Printers.Extensions.Print3DWorkflowStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    enum Print3DWorkflowStatus : int
                    {
                        Print3DWorkflowStatus_Abandoned = 0,
                        Print3DWorkflowStatus_Canceled = 1,
                        Print3DWorkflowStatus_Failed = 2,
                        Print3DWorkflowStatus_Slicing = 3,
                        Print3DWorkflowStatus_Submitted = 4,
                    };
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrint3DWorkflow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.Print3DWorkflow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrint3DWorkflow[] = L"Windows.Devices.Printers.Extensions.IPrint3DWorkflow";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("c56f74bd-3669-4a66-ab42-c8151930cd34")
                    IPrint3DWorkflow : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceID(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetPrintModelPackage(
                            IInspectable** printModelPackage
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsPrintReady(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsPrintReady(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PrintRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs* eventHandler,
                            EventRegistrationToken* eventCookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrintRequested(
                            EventRegistrationToken eventCookie
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrint3DWorkflow = __uuidof(IPrint3DWorkflow);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrint3DWorkflow2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.Print3DWorkflow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrint3DWorkflow2[] = L"Windows.Devices.Printers.Extensions.IPrint3DWorkflow2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("a2a6c54f-8ac1-4918-9741-e34f3004239e")
                    IPrint3DWorkflow2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_PrinterChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs* eventHandler,
                            EventRegistrationToken* eventCookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrinterChanged(
                            EventRegistrationToken eventCookie
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrint3DWorkflow2 = __uuidof(IPrint3DWorkflow2);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrintRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrint3DWorkflowPrintRequestedEventArgs[] = L"Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrintRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("19f8c858-5ac8-4b55-8a5f-e61567dafb4d")
                    IPrint3DWorkflowPrintRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Devices::Printers::Extensions::Print3DWorkflowStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetExtendedStatus(
                            ABI::Windows::Devices::Printers::Extensions::Print3DWorkflowDetail value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetSource(
                            IInspectable* source
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetSourceChanged(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrint3DWorkflowPrintRequestedEventArgs = __uuidof(IPrint3DWorkflowPrintRequestedEventArgs);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrinterChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrint3DWorkflowPrinterChangedEventArgs[] = L"Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrinterChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("45226402-95fc-4847-93b3-134dbf5c60f7")
                    IPrint3DWorkflowPrinterChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_NewDeviceId(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrint3DWorkflowPrinterChangedEventArgs = __uuidof(IPrint3DWorkflowPrinterChangedEventArgs);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintExtensionContextStatic
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintExtensionContext
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintExtensionContextStatic[] = L"Windows.Devices.Printers.Extensions.IPrintExtensionContextStatic";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("e70d9fc1-ff79-4aa4-8c9b-0c93aedfde8a")
                    IPrintExtensionContextStatic : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FromDeviceId(
                            HSTRING deviceId,
                            IInspectable** context
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintExtensionContextStatic = __uuidof(IPrintExtensionContextStatic);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintNotificationEventDetails
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintNotificationEventDetails
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintNotificationEventDetails[] = L"Windows.Devices.Printers.Extensions.IPrintNotificationEventDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("e00e4c8a-4828-4da1-8bb8-8672df8515e7")
                    IPrintNotificationEventDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PrinterName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EventData(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_EventData(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintNotificationEventDetails = __uuidof(IPrintNotificationEventDetails);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintTaskConfiguration
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintTaskConfiguration
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintTaskConfiguration[] = L"Windows.Devices.Printers.Extensions.IPrintTaskConfiguration";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("e3c22451-3aa4-4885-9240-311f5f8fbe9d")
                    IPrintTaskConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PrinterExtensionContext(
                            IInspectable** context
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SaveRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs* eventHandler,
                            EventRegistrationToken* eventCookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SaveRequested(
                            EventRegistrationToken eventCookie
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTaskConfiguration = __uuidof(IPrintTaskConfiguration);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequest
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintTaskConfigurationSaveRequest[] = L"Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("eeaf2fcb-621e-4b62-ac77-b281cce08d60")
                    IPrintTaskConfigurationSaveRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Cancel(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Save(
                            IInspectable* printerExtensionContext
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfigurationSaveRequestedDeferral** deferral
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTaskConfigurationSaveRequest = __uuidof(IPrintTaskConfigurationSaveRequest);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedDeferral
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintTaskConfigurationSaveRequestedDeferral[] = L"Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedDeferral";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("e959d568-f729-44a4-871d-bd0628696a33")
                    IPrintTaskConfigurationSaveRequestedDeferral : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTaskConfigurationSaveRequestedDeferral = __uuidof(IPrintTaskConfigurationSaveRequestedDeferral);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedEventArgs
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintTaskConfigurationSaveRequestedEventArgs[] = L"Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    MIDL_INTERFACE("e06c2879-0d61-4938-91d0-96a45bee8479")
                    IPrintTaskConfigurationSaveRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfigurationSaveRequest** context
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTaskConfigurationSaveRequestedEventArgs = __uuidof(IPrintTaskConfigurationSaveRequestedEventArgs);
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.Print3DWorkflow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrint3DWorkflow ** Default Interface **
 *    Windows.Devices.Printers.Extensions.IPrint3DWorkflow2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflow_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflow_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_Print3DWorkflow[] = L"Windows.Devices.Printers.Extensions.Print3DWorkflow";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrintRequestedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflowPrintRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflowPrintRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_Print3DWorkflowPrintRequestedEventArgs[] = L"Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrinterChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflowPrinterChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflowPrinterChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_Print3DWorkflowPrinterChangedEventArgs[] = L"Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintExtensionContext
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Printers.Extensions.IPrintExtensionContextStatic interface starting with version 1.0 of the Windows.Devices.Printers.Extensions.ExtensionsContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintExtensionContext_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintExtensionContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintExtensionContext[] = L"Windows.Devices.Printers.Extensions.PrintExtensionContext";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintNotificationEventDetails
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintNotificationEventDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintNotificationEventDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintNotificationEventDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintNotificationEventDetails[] = L"Windows.Devices.Printers.Extensions.PrintNotificationEventDetails";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintTaskConfiguration
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintTaskConfiguration ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintTaskConfiguration[] = L"Windows.Devices.Printers.Extensions.PrintTaskConfiguration";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequest ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequest[] = L"Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedDeferral ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedDeferral_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedDeferral[] = L"Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedEventArgs[] = L"Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2 __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* sender,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* sender,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* sender,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef enum __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CPrint3DWorkflowDetail __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CPrint3DWorkflowDetail;

typedef enum __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CPrint3DWorkflowStatus __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CPrint3DWorkflowStatus;

/*
 *
 * Struct Windows.Devices.Printers.Extensions.Print3DWorkflowDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CPrint3DWorkflowDetail
{
    Print3DWorkflowDetail_Unknown = 0,
    Print3DWorkflowDetail_ModelExceedsPrintBed = 1,
    Print3DWorkflowDetail_UploadFailed = 2,
    Print3DWorkflowDetail_InvalidMaterialSelection = 3,
    Print3DWorkflowDetail_InvalidModel = 4,
    Print3DWorkflowDetail_ModelNotManifold = 5,
    Print3DWorkflowDetail_InvalidPrintTicket = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Printers.Extensions.Print3DWorkflowStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CPrint3DWorkflowStatus
{
    Print3DWorkflowStatus_Abandoned = 0,
    Print3DWorkflowStatus_Canceled = 1,
    Print3DWorkflowStatus_Failed = 2,
    Print3DWorkflowStatus_Slicing = 3,
    Print3DWorkflowStatus_Submitted = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrint3DWorkflow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.Print3DWorkflow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrint3DWorkflow[] = L"Windows.Devices.Printers.Extensions.IPrint3DWorkflow";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceID)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetPrintModelPackage)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        IInspectable** printModelPackage);
    HRESULT (STDMETHODCALLTYPE* get_IsPrintReady)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsPrintReady)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* add_PrintRequested)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrintRequestedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_PrintRequested)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_get_DeviceID(This, value) \
    ((This)->lpVtbl->get_DeviceID(This, value))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_GetPrintModelPackage(This, printModelPackage) \
    ((This)->lpVtbl->GetPrintModelPackage(This, printModelPackage))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_get_IsPrintReady(This, value) \
    ((This)->lpVtbl->get_IsPrintReady(This, value))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_put_IsPrintReady(This, value) \
    ((This)->lpVtbl->put_IsPrintReady(This, value))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_add_PrintRequested(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_PrintRequested(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_remove_PrintRequested(This, eventCookie) \
    ((This)->lpVtbl->remove_PrintRequested(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrint3DWorkflow2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.Print3DWorkflow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrint3DWorkflow2[] = L"Windows.Devices.Printers.Extensions.IPrint3DWorkflow2";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PrinterChanged)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2* This,
        __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflow_Windows__CDevices__CPrinters__CExtensions__CPrint3DWorkflowPrinterChangedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_PrinterChanged)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2Vtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_add_PrinterChanged(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_PrinterChanged(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_remove_PrinterChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_PrinterChanged(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrintRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrint3DWorkflowPrintRequestedEventArgs[] = L"Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrintRequestedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CPrint3DWorkflowStatus* value);
    HRESULT (STDMETHODCALLTYPE* SetExtendedStatus)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CPrint3DWorkflowDetail value);
    HRESULT (STDMETHODCALLTYPE* SetSource)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This,
        IInspectable* source);
    HRESULT (STDMETHODCALLTYPE* SetSourceChanged)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_SetExtendedStatus(This, value) \
    ((This)->lpVtbl->SetExtendedStatus(This, value))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_SetSource(This, source) \
    ((This)->lpVtbl->SetSource(This, source))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_SetSourceChanged(This, value) \
    ((This)->lpVtbl->SetSourceChanged(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrintRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrinterChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrint3DWorkflowPrinterChangedEventArgs[] = L"Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrinterChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NewDeviceId)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_get_NewDeviceId(This, value) \
    ((This)->lpVtbl->get_NewDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflowPrinterChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintExtensionContextStatic
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintExtensionContext
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintExtensionContextStatic[] = L"Windows.Devices.Printers.Extensions.IPrintExtensionContextStatic";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStaticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromDeviceId)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic* This,
        HSTRING deviceId,
        IInspectable** context);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStaticVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStaticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_FromDeviceId(This, deviceId, context) \
    ((This)->lpVtbl->FromDeviceId(This, deviceId, context))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintExtensionContextStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintNotificationEventDetails
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintNotificationEventDetails
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintNotificationEventDetails[] = L"Windows.Devices.Printers.Extensions.IPrintNotificationEventDetails";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrinterName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EventData)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_EventData)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_get_PrinterName(This, value) \
    ((This)->lpVtbl->get_PrinterName(This, value))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_get_EventData(This, value) \
    ((This)->lpVtbl->get_EventData(This, value))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_put_EventData(This, value) \
    ((This)->lpVtbl->put_EventData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintNotificationEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintTaskConfiguration
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintTaskConfiguration
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintTaskConfiguration[] = L"Windows.Devices.Printers.Extensions.IPrintTaskConfiguration";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrinterExtensionContext)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This,
        IInspectable** context);
    HRESULT (STDMETHODCALLTYPE* add_SaveRequested)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This,
        __FITypedEventHandler_2_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfiguration_Windows__CDevices__CPrinters__CExtensions__CPrintTaskConfigurationSaveRequestedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_SaveRequested)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_get_PrinterExtensionContext(This, context) \
    ((This)->lpVtbl->get_PrinterExtensionContext(This, context))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_add_SaveRequested(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_SaveRequested(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_remove_SaveRequested(This, eventCookie) \
    ((This)->lpVtbl->remove_SaveRequested(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequest
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintTaskConfigurationSaveRequest[] = L"Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequest";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Cancel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This);
    HRESULT (STDMETHODCALLTYPE* Save)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This,
        IInspectable* printerExtensionContext);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral** deferral);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_Cancel(This) \
    ((This)->lpVtbl->Cancel(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_Save(This, printerExtensionContext) \
    ((This)->lpVtbl->Save(This, printerExtensionContext))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedDeferral
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintTaskConfigurationSaveRequestedDeferral[] = L"Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedDeferral";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferralVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedEventArgs
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Printers_Extensions_IPrintTaskConfigurationSaveRequestedEventArgs[] = L"Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequest** context);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_get_Request(This, context) \
    ((This)->lpVtbl->get_Request(This, context))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfigurationSaveRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.Print3DWorkflow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrint3DWorkflow ** Default Interface **
 *    Windows.Devices.Printers.Extensions.IPrint3DWorkflow2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflow_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflow_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_Print3DWorkflow[] = L"Windows.Devices.Printers.Extensions.Print3DWorkflow";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrintRequestedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflowPrintRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflowPrintRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_Print3DWorkflowPrintRequestedEventArgs[] = L"Windows.Devices.Printers.Extensions.Print3DWorkflowPrintRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrinterChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflowPrinterChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_Print3DWorkflowPrinterChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_Print3DWorkflowPrinterChangedEventArgs[] = L"Windows.Devices.Printers.Extensions.Print3DWorkflowPrinterChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintExtensionContext
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Printers.Extensions.IPrintExtensionContextStatic interface starting with version 1.0 of the Windows.Devices.Printers.Extensions.ExtensionsContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintExtensionContext_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintExtensionContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintExtensionContext[] = L"Windows.Devices.Printers.Extensions.PrintExtensionContext";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintNotificationEventDetails
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintNotificationEventDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintNotificationEventDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintNotificationEventDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintNotificationEventDetails[] = L"Windows.Devices.Printers.Extensions.PrintNotificationEventDetails";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintTaskConfiguration
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintTaskConfiguration ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintTaskConfiguration[] = L"Windows.Devices.Printers.Extensions.PrintTaskConfiguration";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequest ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequest[] = L"Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequest";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedDeferral ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedDeferral_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedDeferral[] = L"Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedDeferral";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs
 *
 * Introduced to Windows.Devices.Printers.Extensions.ExtensionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Printers_Extensions_PrintTaskConfigurationSaveRequestedEventArgs[] = L"Windows.Devices.Printers.Extensions.PrintTaskConfigurationSaveRequestedEventArgs";
#endif
#endif // WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Eprinters2Eextensions_p_h__

#endif // __windows2Edevices2Eprinters2Eextensions_h__
