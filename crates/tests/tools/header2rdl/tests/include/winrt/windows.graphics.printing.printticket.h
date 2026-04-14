
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
#ifndef __windows2Egraphics2Eprinting2Eprintticket_h__
#define __windows2Egraphics2Eprinting2Eprintticket_h__
#ifndef __windows2Egraphics2Eprinting2Eprintticket_p_h__
#define __windows2Egraphics2Eprinting2Eprintticket_p_h__


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
#include "Windows.Data.Xml.Dom.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IPrintTicketCapabilities;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketCapabilities

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IPrintTicketFeature;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IPrintTicketOption;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketOption

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IPrintTicketParameterDefinition;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketParameterDefinition

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IPrintTicketParameterInitializer;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketParameterInitializer

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IPrintTicketValue;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketValue

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IWorkflowPrintTicket;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IWorkflowPrintTicketFactory;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicketFactory

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IWorkflowPrintTicketValidationResult;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicketValidationResult

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class WorkflowPrintTicketValidationResult;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f47c8f81-23ef-5a68-8840-700747b10999"))
IAsyncOperation<ABI::Windows::Graphics::Printing::PrintTicket::WorkflowPrintTicketValidationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTicket::WorkflowPrintTicketValidationResult*, ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicketValidationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Printing::PrintTicket::WorkflowPrintTicketValidationResult*> __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3b0bfff8-0d4b-51eb-b040-493de1addab9"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Printing::PrintTicket::WorkflowPrintTicketValidationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTicket::WorkflowPrintTicketValidationResult*, ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicketValidationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Printing::PrintTicket::WorkflowPrintTicketValidationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class PrintTicketOption;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c92a35e2-829d-5adf-874e-4d745b4ef0aa"))
IIterator<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*, ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketOption*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing.PrintTicket.PrintTicketOption>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*> __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5c7e6676-9046-5b6a-9eb0-c6a954e8226b"))
IIterable<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*, ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketOption*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing.PrintTicket.PrintTicketOption>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*> __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c80b48ef-2a4c-5685-b7a4-88cc796ca274"))
IVectorView<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*, ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketOption*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing.PrintTicket.PrintTicketOption>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketOption*> __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlNode;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode ABI::Windows::Data::Xml::Dom::IXmlNode

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    typedef enum PrintTicketFeatureSelectionType : int PrintTicketFeatureSelectionType;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    typedef enum PrintTicketParameterDataType : int PrintTicketParameterDataType;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    typedef enum PrintTicketValueType : int PrintTicketValueType;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class PrintTicketCapabilities;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class PrintTicketFeature;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class PrintTicketParameterDefinition;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class PrintTicketParameterInitializer;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class PrintTicketValue;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class WorkflowPrintTicket;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Printing.PrintTicket.PrintTicketFeatureSelectionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    enum PrintTicketFeatureSelectionType : int
                    {
                        PrintTicketFeatureSelectionType_PickOne = 0,
                        PrintTicketFeatureSelectionType_PickMany = 1,
                    };
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDataType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    enum PrintTicketParameterDataType : int
                    {
                        PrintTicketParameterDataType_Integer = 0,
                        PrintTicketParameterDataType_NumericString = 1,
                        PrintTicketParameterDataType_String = 2,
                    };
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Graphics.Printing.PrintTicket.PrintTicketValueType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    enum PrintTicketValueType : int
                    {
                        PrintTicketValueType_Integer = 0,
                        PrintTicketValueType_String = 1,
                        PrintTicketValueType_Unknown = 2,
                    };
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketCapabilities[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketCapabilities";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("8c45508b-bbdc-4256-a142-2fd615ecb416")
                    IPrintTicketCapabilities : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNamespace(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNode(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentBindingFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentCollateFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentDuplexFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentHolePunchFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentInputBinFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentNUpFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentStapleFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_JobPasscodeFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageBorderlessFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageMediaSizeFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageMediaTypeFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageOrientationFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageOutputColorFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageOutputQualityFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageResolutionFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetFeature(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetParameterDefinition(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketParameterDefinition** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTicketCapabilities = __uuidof(IPrintTicketCapabilities);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketFeature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketFeature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketFeature[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketFeature";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("e7607d6a-59f5-4103-8858-b97710963d39")
                    IPrintTicketFeature : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNamespace(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNode(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetOption(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketOption** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Options(
                            __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSelectedOption(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketOption** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetSelectedOption(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketOption* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SelectionType(
                            ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketFeatureSelectionType* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTicketFeature = __uuidof(IPrintTicketFeature);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketOption
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketOption[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketOption";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("b086cf90-b367-4e4b-bd48-9c78a0bb31ce")
                    IPrintTicketOption : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNamespace(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNode(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetPropertyNode(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetScoredPropertyNode(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetPropertyValue(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketValue** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetScoredPropertyValue(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketValue** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTicketOption = __uuidof(IPrintTicketOption);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketParameterDefinition[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterDefinition";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("d6bab4e4-2962-4c01-b7f3-9a9294eb8335")
                    IPrintTicketParameterDefinition : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNamespace(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNode(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DataType(
                            ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketParameterDataType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UnitType(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RangeMin(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RangeMax(
                            INT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTicketParameterDefinition = __uuidof(IPrintTicketParameterDefinition);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterInitializer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketParameterInitializer[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterInitializer";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("5e3335bb-a0a5-48b1-9d5c-07116ddc597a")
                    IPrintTicketParameterInitializer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNamespace(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNode(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Value(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketValue* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketValue** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTicketParameterInitializer = __uuidof(IPrintTicketParameterInitializer);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketValue
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketValue[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketValue";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("66b30a32-244d-4e22-a98b-bb3cf1f2dd91")
                    IPrintTicketValue : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Type(
                            ABI::Windows::Graphics::Printing::PrintTicket::PrintTicketValueType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetValueAsInteger(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetValueAsString(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTicketValue = __uuidof(IPrintTicketValue);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IWorkflowPrintTicket[] = L"Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicket";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("41d52285-35e8-448e-a8c5-e4b6a2cf826c")
                    IWorkflowPrintTicket : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNamespace(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XmlNode(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCapabilities(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketCapabilities** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentBindingFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentCollateFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentDuplexFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentHolePunchFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentInputBinFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentNUpFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentStapleFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_JobPasscodeFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageBorderlessFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageMediaSizeFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageMediaTypeFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageOrientationFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageOutputColorFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageOutputQualityFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PageResolutionFeature(
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetFeature(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketFeature** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyXmlChangedAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ValidateAsync(
                            __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetParameterInitializer(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketParameterInitializer** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetParameterInitializerAsInteger(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            INT32 integerValue,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketParameterInitializer** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetParameterInitializerAsString(
                            HSTRING name,
                            HSTRING xmlNamespace,
                            HSTRING stringValue,
                            ABI::Windows::Graphics::Printing::PrintTicket::IPrintTicketParameterInitializer** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE MergeAndValidateTicket(
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket* deltaShemaTicket,
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWorkflowPrintTicket = __uuidof(IWorkflowPrintTicket);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IWorkflowPrintTicketFactory[] = L"Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("966d1166-d9c7-569e-b7d8-f2b341c8f976")
                    IWorkflowPrintTicketFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            HSTRING printerName,
                            ABI::Windows::Storage::Streams::IInputStream* printTicketStream,
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWorkflowPrintTicketFactory = __uuidof(IWorkflowPrintTicketFactory);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IWorkflowPrintTicketValidationResult[] = L"Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketValidationResult";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    MIDL_INTERFACE("0ad1f392-da7b-4a36-bf36-6a99a62e2059")
                    IWorkflowPrintTicketValidationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Validated(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWorkflowPrintTicketValidationResult = __uuidof(IWorkflowPrintTicketValidationResult);
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketCapabilities ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketCapabilities[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketFeature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketFeature ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketFeature_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketFeature_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketFeature[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketFeature";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketOption ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketOption_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketOption_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketOption[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketOption";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterDefinition ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterDefinition_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterDefinition[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterInitializer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterInitializer_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterInitializer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterInitializer[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketValue ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketValue_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketValue[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketValue";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketFactory interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicket ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicket_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicket[] = L"Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketValidationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicketValidationResult_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicketValidationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicketValidationResult[] = L"Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResultVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* This,
        __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        __FIIterator_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketFeatureSelectionType __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketFeatureSelectionType;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketParameterDataType __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketParameterDataType;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketValueType __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketValueType;

/*
 *
 * Struct Windows.Graphics.Printing.PrintTicket.PrintTicketFeatureSelectionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketFeatureSelectionType
{
    PrintTicketFeatureSelectionType_PickOne = 0,
    PrintTicketFeatureSelectionType_PickMany = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDataType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketParameterDataType
{
    PrintTicketParameterDataType_Integer = 0,
    PrintTicketParameterDataType_NumericString = 1,
    PrintTicketParameterDataType_String = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Graphics.Printing.PrintTicket.PrintTicketValueType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketValueType
{
    PrintTicketValueType_Integer = 0,
    PrintTicketValueType_String = 1,
    PrintTicketValueType_Unknown = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketCapabilities[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketCapabilities";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNamespace)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNode)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentBindingFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentCollateFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentDuplexFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentHolePunchFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentInputBinFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentNUpFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentStapleFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_JobPasscodeFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageBorderlessFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageMediaSizeFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageMediaTypeFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageOrientationFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageOutputColorFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageOutputQualityFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageResolutionFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* GetFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** result);
    HRESULT (STDMETHODCALLTYPE* GetParameterDefinition)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilitiesVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_XmlNamespace(This, value) \
    ((This)->lpVtbl->get_XmlNamespace(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_XmlNode(This, value) \
    ((This)->lpVtbl->get_XmlNode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_DocumentBindingFeature(This, value) \
    ((This)->lpVtbl->get_DocumentBindingFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_DocumentCollateFeature(This, value) \
    ((This)->lpVtbl->get_DocumentCollateFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_DocumentDuplexFeature(This, value) \
    ((This)->lpVtbl->get_DocumentDuplexFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_DocumentHolePunchFeature(This, value) \
    ((This)->lpVtbl->get_DocumentHolePunchFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_DocumentInputBinFeature(This, value) \
    ((This)->lpVtbl->get_DocumentInputBinFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_DocumentNUpFeature(This, value) \
    ((This)->lpVtbl->get_DocumentNUpFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_DocumentStapleFeature(This, value) \
    ((This)->lpVtbl->get_DocumentStapleFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_JobPasscodeFeature(This, value) \
    ((This)->lpVtbl->get_JobPasscodeFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_PageBorderlessFeature(This, value) \
    ((This)->lpVtbl->get_PageBorderlessFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_PageMediaSizeFeature(This, value) \
    ((This)->lpVtbl->get_PageMediaSizeFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_PageMediaTypeFeature(This, value) \
    ((This)->lpVtbl->get_PageMediaTypeFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_PageOrientationFeature(This, value) \
    ((This)->lpVtbl->get_PageOrientationFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_PageOutputColorFeature(This, value) \
    ((This)->lpVtbl->get_PageOutputColorFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_PageOutputQualityFeature(This, value) \
    ((This)->lpVtbl->get_PageOutputQualityFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_get_PageResolutionFeature(This, value) \
    ((This)->lpVtbl->get_PageResolutionFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_GetFeature(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetFeature(This, name, xmlNamespace, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_GetParameterDefinition(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetParameterDefinition(This, name, xmlNamespace, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketFeature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketFeature
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketFeature[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketFeature";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeatureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNamespace)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNode)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetOption)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption** result);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintTicket__CPrintTicketOption** result);
    HRESULT (STDMETHODCALLTYPE* GetSelectedOption)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption** value);
    HRESULT (STDMETHODCALLTYPE* SetSelectedOption)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionType)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketFeatureSelectionType* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeatureVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeatureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_get_XmlNamespace(This, value) \
    ((This)->lpVtbl->get_XmlNamespace(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_get_XmlNode(This, value) \
    ((This)->lpVtbl->get_XmlNode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_GetOption(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetOption(This, name, xmlNamespace, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_get_Options(This, result) \
    ((This)->lpVtbl->get_Options(This, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_GetSelectedOption(This, value) \
    ((This)->lpVtbl->GetSelectedOption(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_SetSelectedOption(This, value) \
    ((This)->lpVtbl->SetSelectedOption(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_get_SelectionType(This, value) \
    ((This)->lpVtbl->get_SelectionType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketOption
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketOption[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketOption";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNamespace)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNode)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetPropertyNode)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** result);
    HRESULT (STDMETHODCALLTYPE* GetScoredPropertyNode)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** result);
    HRESULT (STDMETHODCALLTYPE* GetPropertyValue)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue** result);
    HRESULT (STDMETHODCALLTYPE* GetScoredPropertyValue)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOptionVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_get_XmlNamespace(This, value) \
    ((This)->lpVtbl->get_XmlNamespace(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_get_XmlNode(This, value) \
    ((This)->lpVtbl->get_XmlNode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_GetPropertyNode(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetPropertyNode(This, name, xmlNamespace, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_GetScoredPropertyNode(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetScoredPropertyNode(This, name, xmlNamespace, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_GetPropertyValue(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetPropertyValue(This, name, xmlNamespace, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_GetScoredPropertyValue(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetScoredPropertyValue(This, name, xmlNamespace, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketParameterDefinition[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterDefinition";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNamespace)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNode)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_DataType)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketParameterDataType* value);
    HRESULT (STDMETHODCALLTYPE* get_UnitType)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RangeMin)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_RangeMax)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinitionVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_get_XmlNamespace(This, value) \
    ((This)->lpVtbl->get_XmlNamespace(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_get_XmlNode(This, value) \
    ((This)->lpVtbl->get_XmlNode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_get_DataType(This, value) \
    ((This)->lpVtbl->get_DataType(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_get_UnitType(This, value) \
    ((This)->lpVtbl->get_UnitType(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_get_RangeMin(This, value) \
    ((This)->lpVtbl->get_RangeMin(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_get_RangeMax(This, value) \
    ((This)->lpVtbl->get_RangeMax(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterInitializer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketParameterInitializer[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterInitializer";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNamespace)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNode)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializerVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_get_XmlNamespace(This, value) \
    ((This)->lpVtbl->get_XmlNamespace(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_get_XmlNode(This, value) \
    ((This)->lpVtbl->get_XmlNode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IPrintTicketValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.PrintTicketValue
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IPrintTicketValue[] = L"Windows.Graphics.Printing.PrintTicket.IPrintTicketValue";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CPrintTicketValueType* value);
    HRESULT (STDMETHODCALLTYPE* GetValueAsInteger)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* GetValueAsString)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValueVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_GetValueAsInteger(This, value) \
    ((This)->lpVtbl->GetValueAsInteger(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_GetValueAsString(This, value) \
    ((This)->lpVtbl->GetValueAsString(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IWorkflowPrintTicket[] = L"Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicket";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNamespace)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_XmlNode)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* GetCapabilities)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketCapabilities** result);
    HRESULT (STDMETHODCALLTYPE* get_DocumentBindingFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentCollateFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentDuplexFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentHolePunchFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentInputBinFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentNUpFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentStapleFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_JobPasscodeFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageBorderlessFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageMediaSizeFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageMediaTypeFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageOrientationFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageOutputColorFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageOutputQualityFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* get_PageResolutionFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** value);
    HRESULT (STDMETHODCALLTYPE* GetFeature)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketFeature** result);
    HRESULT (STDMETHODCALLTYPE* NotifyXmlChangedAsync)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ValidateAsync)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __FIAsyncOperation_1_Windows__CGraphics__CPrinting__CPrintTicket__CWorkflowPrintTicketValidationResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetParameterInitializer)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        HSTRING name,
        HSTRING xmlNamespace,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer** result);
    HRESULT (STDMETHODCALLTYPE* SetParameterInitializerAsInteger)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        HSTRING name,
        HSTRING xmlNamespace,
        INT32 integerValue,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer** result);
    HRESULT (STDMETHODCALLTYPE* SetParameterInitializerAsString)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        HSTRING name,
        HSTRING xmlNamespace,
        HSTRING stringValue,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIPrintTicketParameterInitializer** result);
    HRESULT (STDMETHODCALLTYPE* MergeAndValidateTicket)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* deltaShemaTicket,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_XmlNamespace(This, value) \
    ((This)->lpVtbl->get_XmlNamespace(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_XmlNode(This, value) \
    ((This)->lpVtbl->get_XmlNode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_GetCapabilities(This, result) \
    ((This)->lpVtbl->GetCapabilities(This, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_DocumentBindingFeature(This, value) \
    ((This)->lpVtbl->get_DocumentBindingFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_DocumentCollateFeature(This, value) \
    ((This)->lpVtbl->get_DocumentCollateFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_DocumentDuplexFeature(This, value) \
    ((This)->lpVtbl->get_DocumentDuplexFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_DocumentHolePunchFeature(This, value) \
    ((This)->lpVtbl->get_DocumentHolePunchFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_DocumentInputBinFeature(This, value) \
    ((This)->lpVtbl->get_DocumentInputBinFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_DocumentNUpFeature(This, value) \
    ((This)->lpVtbl->get_DocumentNUpFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_DocumentStapleFeature(This, value) \
    ((This)->lpVtbl->get_DocumentStapleFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_JobPasscodeFeature(This, value) \
    ((This)->lpVtbl->get_JobPasscodeFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_PageBorderlessFeature(This, value) \
    ((This)->lpVtbl->get_PageBorderlessFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_PageMediaSizeFeature(This, value) \
    ((This)->lpVtbl->get_PageMediaSizeFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_PageMediaTypeFeature(This, value) \
    ((This)->lpVtbl->get_PageMediaTypeFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_PageOrientationFeature(This, value) \
    ((This)->lpVtbl->get_PageOrientationFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_PageOutputColorFeature(This, value) \
    ((This)->lpVtbl->get_PageOutputColorFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_PageOutputQualityFeature(This, value) \
    ((This)->lpVtbl->get_PageOutputQualityFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_get_PageResolutionFeature(This, value) \
    ((This)->lpVtbl->get_PageResolutionFeature(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_GetFeature(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetFeature(This, name, xmlNamespace, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_NotifyXmlChangedAsync(This, operation) \
    ((This)->lpVtbl->NotifyXmlChangedAsync(This, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_ValidateAsync(This, operation) \
    ((This)->lpVtbl->ValidateAsync(This, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_GetParameterInitializer(This, name, xmlNamespace, result) \
    ((This)->lpVtbl->GetParameterInitializer(This, name, xmlNamespace, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_SetParameterInitializerAsInteger(This, name, xmlNamespace, integerValue, result) \
    ((This)->lpVtbl->SetParameterInitializerAsInteger(This, name, xmlNamespace, integerValue, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_SetParameterInitializerAsString(This, name, xmlNamespace, stringValue, result) \
    ((This)->lpVtbl->SetParameterInitializerAsString(This, name, xmlNamespace, stringValue, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_MergeAndValidateTicket(This, deltaShemaTicket, result) \
    ((This)->lpVtbl->MergeAndValidateTicket(This, deltaShemaTicket, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IWorkflowPrintTicketFactory[] = L"Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketFactory";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory* This,
        HSTRING printerName,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* printTicketStream,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_CreateInstance(This, printerName, printTicketStream, value) \
    ((This)->lpVtbl->CreateInstance(This, printerName, printTicketStream, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintTicket_IWorkflowPrintTicketValidationResult[] = L"Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketValidationResult";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Validated)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResultVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_get_Validated(This, value) \
    ((This)->lpVtbl->get_Validated(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicketValidationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketCapabilities ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketCapabilities[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketFeature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketFeature ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketFeature_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketFeature_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketFeature[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketFeature";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketOption ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketOption_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketOption_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketOption[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketOption";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterDefinition ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterDefinition_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterDefinition[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketParameterInitializer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterInitializer_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterInitializer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketParameterInitializer[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.PrintTicketValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IPrintTicketValue ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketValue_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_PrintTicketValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_PrintTicketValue[] = L"Windows.Graphics.Printing.PrintTicket.PrintTicketValue";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketFactory interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicket ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicket_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicket[] = L"Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintTicket.IWorkflowPrintTicketValidationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicketValidationResult_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicketValidationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTicket_WorkflowPrintTicketValidationResult[] = L"Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egraphics2Eprinting2Eprintticket_p_h__

#endif // __windows2Egraphics2Eprinting2Eprintticket_h__
