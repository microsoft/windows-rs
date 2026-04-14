
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
#ifndef __windows2Egraphics2Eprinting_h__
#define __windows2Egraphics2Eprinting_h__
#ifndef __windows2Egraphics2Eprinting_p_h__
#define __windows2Egraphics2Eprinting_p_h__


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
#include "Windows.ApplicationModel.DataTransfer.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskSourceRequestedHandler;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler ABI::Windows::Graphics::Printing::IPrintTaskSourceRequestedHandler

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintDocumentSource;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource ABI::Windows::Graphics::Printing::IPrintDocumentSource

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintManager;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager ABI::Windows::Graphics::Printing::IPrintManager

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintManagerStatic;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic ABI::Windows::Graphics::Printing::IPrintManagerStatic

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintManagerStatic2;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2 ABI::Windows::Graphics::Printing::IPrintManagerStatic2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintPageInfo;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo ABI::Windows::Graphics::Printing::IPrintPageInfo

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintPageRange;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange ABI::Windows::Graphics::Printing::IPrintPageRange

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintPageRangeFactory;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory ABI::Windows::Graphics::Printing::IPrintPageRangeFactory

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintPageRangeOptions;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions ABI::Windows::Graphics::Printing::IPrintPageRangeOptions

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTask;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask ABI::Windows::Graphics::Printing::IPrintTask

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTask2;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2 ABI::Windows::Graphics::Printing::IPrintTask2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskCompletedEventArgs;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs ABI::Windows::Graphics::Printing::IPrintTaskCompletedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskOptions;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions ABI::Windows::Graphics::Printing::IPrintTaskOptions

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskOptions2;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2 ABI::Windows::Graphics::Printing::IPrintTaskOptions2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskOptionsCore;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore ABI::Windows::Graphics::Printing::IPrintTaskOptionsCore

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskOptionsCoreProperties;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties ABI::Windows::Graphics::Printing::IPrintTaskOptionsCoreProperties

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskOptionsCoreUIConfiguration;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration ABI::Windows::Graphics::Printing::IPrintTaskOptionsCoreUIConfiguration

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskProgressingEventArgs;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs ABI::Windows::Graphics::Printing::IPrintTaskProgressingEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskRequest;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest ABI::Windows::Graphics::Printing::IPrintTaskRequest

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskRequestedDeferral;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral ABI::Windows::Graphics::Printing::IPrintTaskRequestedDeferral

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskRequestedEventArgs;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs ABI::Windows::Graphics::Printing::IPrintTaskRequestedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskSourceRequestedArgs;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs ABI::Windows::Graphics::Printing::IPrintTaskSourceRequestedArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskSourceRequestedDeferral;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral ABI::Windows::Graphics::Printing::IPrintTaskSourceRequestedDeferral

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskTargetDeviceSupport;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport ABI::Windows::Graphics::Printing::IPrintTaskTargetDeviceSupport

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IStandardPrintTaskOptionsStatic;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic ABI::Windows::Graphics::Printing::IStandardPrintTaskOptionsStatic

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IStandardPrintTaskOptionsStatic2;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2 ABI::Windows::Graphics::Printing::IStandardPrintTaskOptionsStatic2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IStandardPrintTaskOptionsStatic3;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3 ABI::Windows::Graphics::Printing::IStandardPrintTaskOptionsStatic3

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_FWD_DEFINED__

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
        namespace Graphics {
            namespace Printing {
                class PrintPageRange;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("38f75665-fb40-53e2-a916-15ba754a6a9e"))
IIterator<ABI::Windows::Graphics::Printing::PrintPageRange*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintPageRange*, ABI::Windows::Graphics::Printing::IPrintPageRange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing.PrintPageRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing::PrintPageRange*> __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ed6ddf38-34cf-5a2f-b9ac-413dffce7f81"))
IIterable<ABI::Windows::Graphics::Printing::PrintPageRange*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintPageRange*, ABI::Windows::Graphics::Printing::IPrintPageRange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing.PrintPageRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing::PrintPageRange*> __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0063b11e-2098-5eff-9652-08a3516c99a6"))
IVectorView<ABI::Windows::Graphics::Printing::PrintPageRange*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintPageRange*, ABI::Windows::Graphics::Printing::IPrintPageRange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing.PrintPageRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing::PrintPageRange*> __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1d314a18-2a16-542f-a986-1b7841b878aa"))
IVector<ABI::Windows::Graphics::Printing::PrintPageRange*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintPageRange*, ABI::Windows::Graphics::Printing::IPrintPageRange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing.PrintPageRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing::PrintPageRange*> __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_t;
#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintManager;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskRequestedEventArgs;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8a8cb877-70c5-54ce-8b42-d790e2914859"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintManager*, ABI::Windows::Graphics::Printing::PrintTaskRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintManager*, ABI::Windows::Graphics::Printing::IPrintManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTaskRequestedEventArgs*, ABI::Windows::Graphics::Printing::IPrintTaskRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.PrintManager, Windows.Graphics.Printing.PrintTaskRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintManager*, ABI::Windows::Graphics::Printing::PrintTaskRequestedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTask;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4cc141d4-c0d9-5220-b1ce-80fff3bd2d44"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintTask*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTask*, ABI::Windows::Graphics::Printing::IPrintTask*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.PrintTask, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintTask*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskCompletedEventArgs;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b0b02549-b9ad-5226-898a-7b563b46640c"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintTask*, ABI::Windows::Graphics::Printing::PrintTaskCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTask*, ABI::Windows::Graphics::Printing::IPrintTask*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTaskCompletedEventArgs*, ABI::Windows::Graphics::Printing::IPrintTaskCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.PrintTask, Windows.Graphics.Printing.PrintTaskCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintTask*, ABI::Windows::Graphics::Printing::PrintTaskCompletedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskProgressingEventArgs;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c08d0524-5899-536c-8f46-55fdaa4cf78b"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintTask*, ABI::Windows::Graphics::Printing::PrintTaskProgressingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTask*, ABI::Windows::Graphics::Printing::IPrintTask*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintTaskProgressingEventArgs*, ABI::Windows::Graphics::Printing::IPrintTaskProgressingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.PrintTask, Windows.Graphics.Printing.PrintTaskProgressingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintTask*, ABI::Windows::Graphics::Printing::PrintTaskProgressingEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataPackagePropertySet;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySet;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySet

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintBinding : int PrintBinding;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintBordering : int PrintBordering;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintCollation : int PrintCollation;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintColorMode : int PrintColorMode;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintDuplex : int PrintDuplex;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintHolePunch : int PrintHolePunch;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintMediaSize : int PrintMediaSize;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintMediaType : int PrintMediaType;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintOrientation : int PrintOrientation;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintQuality : int PrintQuality;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintStaple : int PrintStaple;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef enum PrintTaskCompletion : int PrintTaskCompletion;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                typedef struct PrintPageDescription PrintPageDescription;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintPageInfo;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintPageRangeOptions;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskOptions;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskRequest;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskRequestedDeferral;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskSourceRequestedArgs;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskSourceRequestedDeferral;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Printing.PrintBinding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintBinding : int
                {
                    PrintBinding_Default = 0,
                    PrintBinding_NotAvailable = 1,
                    PrintBinding_PrinterCustom = 2,
                    PrintBinding_None = 3,
                    PrintBinding_Bale = 4,
                    PrintBinding_BindBottom = 5,
                    PrintBinding_BindLeft = 6,
                    PrintBinding_BindRight = 7,
                    PrintBinding_BindTop = 8,
                    PrintBinding_Booklet = 9,
                    PrintBinding_EdgeStitchBottom = 10,
                    PrintBinding_EdgeStitchLeft = 11,
                    PrintBinding_EdgeStitchRight = 12,
                    PrintBinding_EdgeStitchTop = 13,
                    PrintBinding_Fold = 14,
                    PrintBinding_JogOffset = 15,
                    PrintBinding_Trim = 16,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintBordering
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintBordering : int
                {
                    PrintBordering_Default = 0,
                    PrintBordering_NotAvailable = 1,
                    PrintBordering_PrinterCustom = 2,
                    PrintBordering_Bordered = 3,
                    PrintBordering_Borderless = 4,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Graphics.Printing.PrintCollation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintCollation : int
                {
                    PrintCollation_Default = 0,
                    PrintCollation_NotAvailable = 1,
                    PrintCollation_PrinterCustom = 2,
                    PrintCollation_Collated = 3,
                    PrintCollation_Uncollated = 4,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintColorMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintColorMode : int
                {
                    PrintColorMode_Default = 0,
                    PrintColorMode_NotAvailable = 1,
                    PrintColorMode_PrinterCustom = 2,
                    PrintColorMode_Color = 3,
                    PrintColorMode_Grayscale = 4,
                    PrintColorMode_Monochrome = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
                    PrintColorMode_AutoSelect = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintDuplex
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintDuplex : int
                {
                    PrintDuplex_Default = 0,
                    PrintDuplex_NotAvailable = 1,
                    PrintDuplex_PrinterCustom = 2,
                    PrintDuplex_OneSided = 3,
                    PrintDuplex_TwoSidedShortEdge = 4,
                    PrintDuplex_TwoSidedLongEdge = 5,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintHolePunch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintHolePunch : int
                {
                    PrintHolePunch_Default = 0,
                    PrintHolePunch_NotAvailable = 1,
                    PrintHolePunch_PrinterCustom = 2,
                    PrintHolePunch_None = 3,
                    PrintHolePunch_LeftEdge = 4,
                    PrintHolePunch_RightEdge = 5,
                    PrintHolePunch_TopEdge = 6,
                    PrintHolePunch_BottomEdge = 7,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintMediaSize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintMediaSize : int
                {
                    PrintMediaSize_Default = 0,
                    PrintMediaSize_NotAvailable = 1,
                    PrintMediaSize_PrinterCustom = 2,
                    PrintMediaSize_BusinessCard = 3,
                    PrintMediaSize_CreditCard = 4,
                    PrintMediaSize_IsoA0 = 5,
                    PrintMediaSize_IsoA1 = 6,
                    PrintMediaSize_IsoA10 = 7,
                    PrintMediaSize_IsoA2 = 8,
                    PrintMediaSize_IsoA3 = 9,
                    PrintMediaSize_IsoA3Extra = 10,
                    PrintMediaSize_IsoA3Rotated = 11,
                    PrintMediaSize_IsoA4 = 12,
                    PrintMediaSize_IsoA4Extra = 13,
                    PrintMediaSize_IsoA4Rotated = 14,
                    PrintMediaSize_IsoA5 = 15,
                    PrintMediaSize_IsoA5Extra = 16,
                    PrintMediaSize_IsoA5Rotated = 17,
                    PrintMediaSize_IsoA6 = 18,
                    PrintMediaSize_IsoA6Rotated = 19,
                    PrintMediaSize_IsoA7 = 20,
                    PrintMediaSize_IsoA8 = 21,
                    PrintMediaSize_IsoA9 = 22,
                    PrintMediaSize_IsoB0 = 23,
                    PrintMediaSize_IsoB1 = 24,
                    PrintMediaSize_IsoB10 = 25,
                    PrintMediaSize_IsoB2 = 26,
                    PrintMediaSize_IsoB3 = 27,
                    PrintMediaSize_IsoB4 = 28,
                    PrintMediaSize_IsoB4Envelope = 29,
                    PrintMediaSize_IsoB5Envelope = 30,
                    PrintMediaSize_IsoB5Extra = 31,
                    PrintMediaSize_IsoB7 = 32,
                    PrintMediaSize_IsoB8 = 33,
                    PrintMediaSize_IsoB9 = 34,
                    PrintMediaSize_IsoC0 = 35,
                    PrintMediaSize_IsoC1 = 36,
                    PrintMediaSize_IsoC10 = 37,
                    PrintMediaSize_IsoC2 = 38,
                    PrintMediaSize_IsoC3 = 39,
                    PrintMediaSize_IsoC3Envelope = 40,
                    PrintMediaSize_IsoC4 = 41,
                    PrintMediaSize_IsoC4Envelope = 42,
                    PrintMediaSize_IsoC5 = 43,
                    PrintMediaSize_IsoC5Envelope = 44,
                    PrintMediaSize_IsoC6 = 45,
                    PrintMediaSize_IsoC6C5Envelope = 46,
                    PrintMediaSize_IsoC6Envelope = 47,
                    PrintMediaSize_IsoC7 = 48,
                    PrintMediaSize_IsoC8 = 49,
                    PrintMediaSize_IsoC9 = 50,
                    PrintMediaSize_IsoDLEnvelope = 51,
                    PrintMediaSize_IsoDLEnvelopeRotated = 52,
                    PrintMediaSize_IsoSRA3 = 53,
                    PrintMediaSize_Japan2LPhoto = 54,
                    PrintMediaSize_JapanChou3Envelope = 55,
                    PrintMediaSize_JapanChou3EnvelopeRotated = 56,
                    PrintMediaSize_JapanChou4Envelope = 57,
                    PrintMediaSize_JapanChou4EnvelopeRotated = 58,
                    PrintMediaSize_JapanDoubleHagakiPostcard = 59,
                    PrintMediaSize_JapanDoubleHagakiPostcardRotated = 60,
                    PrintMediaSize_JapanHagakiPostcard = 61,
                    PrintMediaSize_JapanHagakiPostcardRotated = 62,
                    PrintMediaSize_JapanKaku2Envelope = 63,
                    PrintMediaSize_JapanKaku2EnvelopeRotated = 64,
                    PrintMediaSize_JapanKaku3Envelope = 65,
                    PrintMediaSize_JapanKaku3EnvelopeRotated = 66,
                    PrintMediaSize_JapanLPhoto = 67,
                    PrintMediaSize_JapanQuadrupleHagakiPostcard = 68,
                    PrintMediaSize_JapanYou1Envelope = 69,
                    PrintMediaSize_JapanYou2Envelope = 70,
                    PrintMediaSize_JapanYou3Envelope = 71,
                    PrintMediaSize_JapanYou4Envelope = 72,
                    PrintMediaSize_JapanYou4EnvelopeRotated = 73,
                    PrintMediaSize_JapanYou6Envelope = 74,
                    PrintMediaSize_JapanYou6EnvelopeRotated = 75,
                    PrintMediaSize_JisB0 = 76,
                    PrintMediaSize_JisB1 = 77,
                    PrintMediaSize_JisB10 = 78,
                    PrintMediaSize_JisB2 = 79,
                    PrintMediaSize_JisB3 = 80,
                    PrintMediaSize_JisB4 = 81,
                    PrintMediaSize_JisB4Rotated = 82,
                    PrintMediaSize_JisB5 = 83,
                    PrintMediaSize_JisB5Rotated = 84,
                    PrintMediaSize_JisB6 = 85,
                    PrintMediaSize_JisB6Rotated = 86,
                    PrintMediaSize_JisB7 = 87,
                    PrintMediaSize_JisB8 = 88,
                    PrintMediaSize_JisB9 = 89,
                    PrintMediaSize_NorthAmerica10x11 = 90,
                    PrintMediaSize_NorthAmerica10x12 = 91,
                    PrintMediaSize_NorthAmerica10x14 = 92,
                    PrintMediaSize_NorthAmerica11x17 = 93,
                    PrintMediaSize_NorthAmerica14x17 = 94,
                    PrintMediaSize_NorthAmerica4x6 = 95,
                    PrintMediaSize_NorthAmerica4x8 = 96,
                    PrintMediaSize_NorthAmerica5x7 = 97,
                    PrintMediaSize_NorthAmerica8x10 = 98,
                    PrintMediaSize_NorthAmerica9x11 = 99,
                    PrintMediaSize_NorthAmericaArchitectureASheet = 100,
                    PrintMediaSize_NorthAmericaArchitectureBSheet = 101,
                    PrintMediaSize_NorthAmericaArchitectureCSheet = 102,
                    PrintMediaSize_NorthAmericaArchitectureDSheet = 103,
                    PrintMediaSize_NorthAmericaArchitectureESheet = 104,
                    PrintMediaSize_NorthAmericaCSheet = 105,
                    PrintMediaSize_NorthAmericaDSheet = 106,
                    PrintMediaSize_NorthAmericaESheet = 107,
                    PrintMediaSize_NorthAmericaExecutive = 108,
                    PrintMediaSize_NorthAmericaGermanLegalFanfold = 109,
                    PrintMediaSize_NorthAmericaGermanStandardFanfold = 110,
                    PrintMediaSize_NorthAmericaLegal = 111,
                    PrintMediaSize_NorthAmericaLegalExtra = 112,
                    PrintMediaSize_NorthAmericaLetter = 113,
                    PrintMediaSize_NorthAmericaLetterExtra = 114,
                    PrintMediaSize_NorthAmericaLetterPlus = 115,
                    PrintMediaSize_NorthAmericaLetterRotated = 116,
                    PrintMediaSize_NorthAmericaMonarchEnvelope = 117,
                    PrintMediaSize_NorthAmericaNote = 118,
                    PrintMediaSize_NorthAmericaNumber10Envelope = 119,
                    PrintMediaSize_NorthAmericaNumber10EnvelopeRotated = 120,
                    PrintMediaSize_NorthAmericaNumber11Envelope = 121,
                    PrintMediaSize_NorthAmericaNumber12Envelope = 122,
                    PrintMediaSize_NorthAmericaNumber14Envelope = 123,
                    PrintMediaSize_NorthAmericaNumber9Envelope = 124,
                    PrintMediaSize_NorthAmericaPersonalEnvelope = 125,
                    PrintMediaSize_NorthAmericaQuarto = 126,
                    PrintMediaSize_NorthAmericaStatement = 127,
                    PrintMediaSize_NorthAmericaSuperA = 128,
                    PrintMediaSize_NorthAmericaSuperB = 129,
                    PrintMediaSize_NorthAmericaTabloid = 130,
                    PrintMediaSize_NorthAmericaTabloidExtra = 131,
                    PrintMediaSize_OtherMetricA3Plus = 132,
                    PrintMediaSize_OtherMetricA4Plus = 133,
                    PrintMediaSize_OtherMetricFolio = 134,
                    PrintMediaSize_OtherMetricInviteEnvelope = 135,
                    PrintMediaSize_OtherMetricItalianEnvelope = 136,
                    PrintMediaSize_Prc10Envelope = 137,
                    PrintMediaSize_Prc10EnvelopeRotated = 138,
                    PrintMediaSize_Prc16K = 139,
                    PrintMediaSize_Prc16KRotated = 140,
                    PrintMediaSize_Prc1Envelope = 141,
                    PrintMediaSize_Prc1EnvelopeRotated = 142,
                    PrintMediaSize_Prc2Envelope = 143,
                    PrintMediaSize_Prc2EnvelopeRotated = 144,
                    PrintMediaSize_Prc32K = 145,
                    PrintMediaSize_Prc32KBig = 146,
                    PrintMediaSize_Prc32KRotated = 147,
                    PrintMediaSize_Prc3Envelope = 148,
                    PrintMediaSize_Prc3EnvelopeRotated = 149,
                    PrintMediaSize_Prc4Envelope = 150,
                    PrintMediaSize_Prc4EnvelopeRotated = 151,
                    PrintMediaSize_Prc5Envelope = 152,
                    PrintMediaSize_Prc5EnvelopeRotated = 153,
                    PrintMediaSize_Prc6Envelope = 154,
                    PrintMediaSize_Prc6EnvelopeRotated = 155,
                    PrintMediaSize_Prc7Envelope = 156,
                    PrintMediaSize_Prc7EnvelopeRotated = 157,
                    PrintMediaSize_Prc8Envelope = 158,
                    PrintMediaSize_Prc8EnvelopeRotated = 159,
                    PrintMediaSize_Prc9Envelope = 160,
                    PrintMediaSize_Prc9EnvelopeRotated = 161,
                    PrintMediaSize_Roll04Inch = 162,
                    PrintMediaSize_Roll06Inch = 163,
                    PrintMediaSize_Roll08Inch = 164,
                    PrintMediaSize_Roll12Inch = 165,
                    PrintMediaSize_Roll15Inch = 166,
                    PrintMediaSize_Roll18Inch = 167,
                    PrintMediaSize_Roll22Inch = 168,
                    PrintMediaSize_Roll24Inch = 169,
                    PrintMediaSize_Roll30Inch = 170,
                    PrintMediaSize_Roll36Inch = 171,
                    PrintMediaSize_Roll54Inch = 172,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintMediaType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintMediaType : int
                {
                    PrintMediaType_Default = 0,
                    PrintMediaType_NotAvailable = 1,
                    PrintMediaType_PrinterCustom = 2,
                    PrintMediaType_AutoSelect = 3,
                    PrintMediaType_Archival = 4,
                    PrintMediaType_BackPrintFilm = 5,
                    PrintMediaType_Bond = 6,
                    PrintMediaType_CardStock = 7,
                    PrintMediaType_Continuous = 8,
                    PrintMediaType_EnvelopePlain = 9,
                    PrintMediaType_EnvelopeWindow = 10,
                    PrintMediaType_Fabric = 11,
                    PrintMediaType_HighResolution = 12,
                    PrintMediaType_Label = 13,
                    PrintMediaType_MultiLayerForm = 14,
                    PrintMediaType_MultiPartForm = 15,
                    PrintMediaType_Photographic = 16,
                    PrintMediaType_PhotographicFilm = 17,
                    PrintMediaType_PhotographicGlossy = 18,
                    PrintMediaType_PhotographicHighGloss = 19,
                    PrintMediaType_PhotographicMatte = 20,
                    PrintMediaType_PhotographicSatin = 21,
                    PrintMediaType_PhotographicSemiGloss = 22,
                    PrintMediaType_Plain = 23,
                    PrintMediaType_Screen = 24,
                    PrintMediaType_ScreenPaged = 25,
                    PrintMediaType_Stationery = 26,
                    PrintMediaType_TabStockFull = 27,
                    PrintMediaType_TabStockPreCut = 28,
                    PrintMediaType_Transparency = 29,
                    PrintMediaType_TShirtTransfer = 30,
                    PrintMediaType_None = 31,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintOrientation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintOrientation : int
                {
                    PrintOrientation_Default = 0,
                    PrintOrientation_NotAvailable = 1,
                    PrintOrientation_PrinterCustom = 2,
                    PrintOrientation_Portrait = 3,
                    PrintOrientation_PortraitFlipped = 4,
                    PrintOrientation_Landscape = 5,
                    PrintOrientation_LandscapeFlipped = 6,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintQuality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintQuality : int
                {
                    PrintQuality_Default = 0,
                    PrintQuality_NotAvailable = 1,
                    PrintQuality_PrinterCustom = 2,
                    PrintQuality_Automatic = 3,
                    PrintQuality_Draft = 4,
                    PrintQuality_Fax = 5,
                    PrintQuality_High = 6,
                    PrintQuality_Normal = 7,
                    PrintQuality_Photographic = 8,
                    PrintQuality_Text = 9,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintStaple
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintStaple : int
                {
                    PrintStaple_Default = 0,
                    PrintStaple_NotAvailable = 1,
                    PrintStaple_PrinterCustom = 2,
                    PrintStaple_None = 3,
                    PrintStaple_StapleTopLeft = 4,
                    PrintStaple_StapleTopRight = 5,
                    PrintStaple_StapleBottomLeft = 6,
                    PrintStaple_StapleBottomRight = 7,
                    PrintStaple_StapleDualLeft = 8,
                    PrintStaple_StapleDualRight = 9,
                    PrintStaple_StapleDualTop = 10,
                    PrintStaple_StapleDualBottom = 11,
                    PrintStaple_SaddleStitch = 12,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintTaskCompletion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                enum PrintTaskCompletion : int
                {
                    PrintTaskCompletion_Abandoned = 0,
                    PrintTaskCompletion_Canceled = 1,
                    PrintTaskCompletion_Failed = 2,
                    PrintTaskCompletion_Submitted = 3,
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintPageDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                struct PrintPageDescription
                {
                    ABI::Windows::Foundation::Size PageSize;
                    ABI::Windows::Foundation::Rect ImageableRect;
                    UINT32 DpiX;
                    UINT32 DpiY;
                };
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Graphics.Printing.PrintTaskSourceRequestedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("6c109fa8-5cb6-4b3a-8663-f39cb02dc9b4")
                IPrintTaskSourceRequestedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Graphics::Printing::IPrintTaskSourceRequestedArgs* args
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskSourceRequestedHandler = __uuidof(IPrintTaskSourceRequestedHandler);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintDocumentSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintDocumentSource[] = L"Windows.Graphics.Printing.IPrintDocumentSource";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("dedc0c30-f1eb-47df-aae6-ed5427511f01")
                IPrintDocumentSource : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IPrintDocumentSource = __uuidof(IPrintDocumentSource);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintManager[] = L"Windows.Graphics.Printing.IPrintManager";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("ff2a9694-8c99-44fd-ae4a-19d9aa9a0f0a")
                IPrintManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_PrintTaskRequested(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PrintTaskRequested(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintManager = __uuidof(IPrintManager);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintManagerStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintManagerStatic[] = L"Windows.Graphics.Printing.IPrintManagerStatic";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("58185dcd-e634-4654-84f0-e0152a8217ac")
                IPrintManagerStatic : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::Graphics::Printing::IPrintManager** printingManager
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowPrintUIAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintManagerStatic = __uuidof(IPrintManagerStatic);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintManagerStatic2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintManagerStatic2[] = L"Windows.Graphics.Printing.IPrintManagerStatic2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("35a99955-e6ab-4139-9abd-b86a729b3598")
                IPrintManagerStatic2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintManagerStatic2 = __uuidof(IPrintManagerStatic2);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintPageInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintPageInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintPageInfo[] = L"Windows.Graphics.Printing.IPrintPageInfo";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("dd4be9c9-a6a1-4ada-930e-da872a4f23d3")
                IPrintPageInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_MediaSize(
                        ABI::Windows::Graphics::Printing::PrintMediaSize value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaSize(
                        ABI::Windows::Graphics::Printing::PrintMediaSize* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PageSize(
                        ABI::Windows::Foundation::Size value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PageSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DpiX(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DpiX(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DpiY(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DpiY(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Orientation(
                        ABI::Windows::Graphics::Printing::PrintOrientation value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        ABI::Windows::Graphics::Printing::PrintOrientation* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintPageInfo = __uuidof(IPrintPageInfo);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintPageRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintPageRange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintPageRange[] = L"Windows.Graphics.Printing.IPrintPageRange";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("f8a06c54-6e7c-51c5-57fd-0660c2d71513")
                IPrintPageRange : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FirstPageNumber(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastPageNumber(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintPageRange = __uuidof(IPrintPageRange);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintPageRangeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintPageRange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintPageRangeFactory[] = L"Windows.Graphics.Printing.IPrintPageRangeFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("408fd45f-e047-5f85-7129-fb085a4fad14")
                IPrintPageRangeFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        INT32 firstPage,
                        INT32 lastPage,
                        ABI::Windows::Graphics::Printing::IPrintPageRange** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithSinglePage(
                        INT32 page,
                        ABI::Windows::Graphics::Printing::IPrintPageRange** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintPageRangeFactory = __uuidof(IPrintPageRangeFactory);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintPageRangeOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintPageRangeOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintPageRangeOptions[] = L"Windows.Graphics.Printing.IPrintPageRangeOptions";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("ce6db728-1357-46b2-a923-79f995f448fc")
                IPrintPageRangeOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_AllowAllPages(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowAllPages(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowCurrentPage(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowCurrentPage(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowCustomSetOfPages(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowCustomSetOfPages(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintPageRangeOptions = __uuidof(IPrintPageRangeOptions);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTask[] = L"Windows.Graphics.Printing.IPrintTask";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("61d80247-6cf6-4fad-84e2-a5e82e2d4ceb")
                IPrintTask : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::Graphics::Printing::IPrintDocumentSource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Options(
                        ABI::Windows::Graphics::Printing::IPrintTaskOptionsCore** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Previewing(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Previewing(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Submitting(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Submitting(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Progressing(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Progressing(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Completed(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Completed(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTask = __uuidof(IPrintTask);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTask2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTask2[] = L"Windows.Graphics.Printing.IPrintTask2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("36234877-3e53-4d9d-8f5e-316ac8dedae1")
                IPrintTask2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_IsPreviewEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPreviewEnabled(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTask2 = __uuidof(IPrintTask2);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskCompletedEventArgs[] = L"Windows.Graphics.Printing.IPrintTaskCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("5bcd34af-24e9-4c10-8d07-14c346ba3fce")
                IPrintTaskCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Completion(
                        ABI::Windows::Graphics::Printing::PrintTaskCompletion* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskCompletedEventArgs = __uuidof(IPrintTaskCompletedEventArgs);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptions[] = L"Windows.Graphics.Printing.IPrintTaskOptions";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("5a0a66bb-d289-41bb-96dd-57e28338ae3f")
                IPrintTaskOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Bordering(
                        ABI::Windows::Graphics::Printing::PrintBordering value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bordering(
                        ABI::Windows::Graphics::Printing::PrintBordering* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPagePrintTicket(
                        ABI::Windows::Graphics::Printing::IPrintPageInfo* printPageInfo,
                        ABI::Windows::Storage::Streams::IRandomAccessStream** printTicket
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskOptions = __uuidof(IPrintTaskOptions);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptions2[] = L"Windows.Graphics.Printing.IPrintTaskOptions2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("eb9b1606-9a36-4b59-8617-b217849262e1")
                IPrintTaskOptions2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PageRangeOptions(
                        ABI::Windows::Graphics::Printing::IPrintPageRangeOptions** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CustomPageRanges(
                        __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskOptions2 = __uuidof(IPrintTaskOptions2);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptionsCore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptionsCore[] = L"Windows.Graphics.Printing.IPrintTaskOptionsCore";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("1bdbb474-4ed1-41eb-be3c-72d18ed67337")
                IPrintTaskOptionsCore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPageDescription(
                        UINT32 jobPageNumber,
                        ABI::Windows::Graphics::Printing::PrintPageDescription* description
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskOptionsCore = __uuidof(IPrintTaskOptionsCore);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptionsCoreProperties[] = L"Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("c1b71832-9e93-4e55-814b-3326a59efce1")
                IPrintTaskOptionsCoreProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_MediaSize(
                        ABI::Windows::Graphics::Printing::PrintMediaSize value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaSize(
                        ABI::Windows::Graphics::Printing::PrintMediaSize* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MediaType(
                        ABI::Windows::Graphics::Printing::PrintMediaType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaType(
                        ABI::Windows::Graphics::Printing::PrintMediaType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Orientation(
                        ABI::Windows::Graphics::Printing::PrintOrientation value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        ABI::Windows::Graphics::Printing::PrintOrientation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PrintQuality(
                        ABI::Windows::Graphics::Printing::PrintQuality value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrintQuality(
                        ABI::Windows::Graphics::Printing::PrintQuality* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ColorMode(
                        ABI::Windows::Graphics::Printing::PrintColorMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ColorMode(
                        ABI::Windows::Graphics::Printing::PrintColorMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Duplex(
                        ABI::Windows::Graphics::Printing::PrintDuplex value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duplex(
                        ABI::Windows::Graphics::Printing::PrintDuplex* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Collation(
                        ABI::Windows::Graphics::Printing::PrintCollation value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Collation(
                        ABI::Windows::Graphics::Printing::PrintCollation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Staple(
                        ABI::Windows::Graphics::Printing::PrintStaple value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Staple(
                        ABI::Windows::Graphics::Printing::PrintStaple* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HolePunch(
                        ABI::Windows::Graphics::Printing::PrintHolePunch value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HolePunch(
                        ABI::Windows::Graphics::Printing::PrintHolePunch* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Binding(
                        ABI::Windows::Graphics::Printing::PrintBinding value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Binding(
                        ABI::Windows::Graphics::Printing::PrintBinding* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinCopies(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxCopies(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NumberOfCopies(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NumberOfCopies(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskOptionsCoreProperties = __uuidof(IPrintTaskOptionsCoreProperties);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptionsCoreUIConfiguration[] = L"Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("62e69e23-9a1e-4336-b74f-3cc7f4cff709")
                IPrintTaskOptionsCoreUIConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayedOptions(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskOptionsCoreUIConfiguration = __uuidof(IPrintTaskOptionsCoreUIConfiguration);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskProgressingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskProgressingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskProgressingEventArgs[] = L"Windows.Graphics.Printing.IPrintTaskProgressingEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("810cd3cb-b410-4282-a073-5ac378234174")
                IPrintTaskProgressingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DocumentPageCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskProgressingEventArgs = __uuidof(IPrintTaskProgressingEventArgs);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskRequest[] = L"Windows.Graphics.Printing.IPrintTaskRequest";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("6ff61e2e-2722-4240-a67c-f364849a17f3")
                IPrintTaskRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePrintTask(
                        HSTRING title,
                        ABI::Windows::Graphics::Printing::IPrintTaskSourceRequestedHandler* handler,
                        ABI::Windows::Graphics::Printing::IPrintTask** task
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Graphics::Printing::IPrintTaskRequestedDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskRequest = __uuidof(IPrintTaskRequest);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskRequestedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskRequestedDeferral[] = L"Windows.Graphics.Printing.IPrintTaskRequestedDeferral";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("cfefb3f0-ce3e-42c7-9496-64800c622c44")
                IPrintTaskRequestedDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskRequestedDeferral = __uuidof(IPrintTaskRequestedDeferral);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskRequestedEventArgs[] = L"Windows.Graphics.Printing.IPrintTaskRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("d0aff924-a31b-454c-a7b6-5d0cc522fc16")
                IPrintTaskRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        ABI::Windows::Graphics::Printing::IPrintTaskRequest** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskRequestedEventArgs = __uuidof(IPrintTaskRequestedEventArgs);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskSourceRequestedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskSourceRequestedArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskSourceRequestedArgs[] = L"Windows.Graphics.Printing.IPrintTaskSourceRequestedArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("f9f067be-f456-41f0-9c98-5ce73e851410")
                IPrintTaskSourceRequestedArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSource(
                        ABI::Windows::Graphics::Printing::IPrintDocumentSource* source
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Graphics::Printing::IPrintTaskSourceRequestedDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskSourceRequestedArgs = __uuidof(IPrintTaskSourceRequestedArgs);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskSourceRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskSourceRequestedDeferral[] = L"Windows.Graphics.Printing.IPrintTaskSourceRequestedDeferral";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("4a1560d1-6992-4d9d-8555-4ca4563fb166")
                IPrintTaskSourceRequestedDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskSourceRequestedDeferral = __uuidof(IPrintTaskSourceRequestedDeferral);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskTargetDeviceSupport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskTargetDeviceSupport[] = L"Windows.Graphics.Printing.IPrintTaskTargetDeviceSupport";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("295d70c0-c2cb-4b7d-b0ea-93095091a220")
                IPrintTaskTargetDeviceSupport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_IsPrinterTargetEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPrinterTargetEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Is3DManufacturingTargetEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Is3DManufacturingTargetEnabled(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskTargetDeviceSupport = __uuidof(IPrintTaskTargetDeviceSupport);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.StandardPrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IStandardPrintTaskOptionsStatic[] = L"Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("b4483d26-0dd0-4cd4-baff-930fc7d6a574")
                IStandardPrintTaskOptionsStatic : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MediaSize(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaType(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrintQuality(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ColorMode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duplex(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Collation(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Staple(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HolePunch(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Binding(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Copies(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NUp(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputBin(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStandardPrintTaskOptionsStatic = __uuidof(IStandardPrintTaskOptionsStatic);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.StandardPrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IStandardPrintTaskOptionsStatic2[] = L"Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("3be38bf4-7a44-4269-9a52-81261e289ee9")
                IStandardPrintTaskOptionsStatic2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Bordering(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStandardPrintTaskOptionsStatic2 = __uuidof(IStandardPrintTaskOptionsStatic2);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.StandardPrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IStandardPrintTaskOptionsStatic3[] = L"Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                MIDL_INTERFACE("bbf68e86-3858-41b3-a799-55dd9888d475")
                IStandardPrintTaskOptionsStatic3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CustomPageRanges(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStandardPrintTaskOptionsStatic3 = __uuidof(IStandardPrintTaskOptionsStatic3);
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.PrintManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.IPrintManagerStatic2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Printing.IPrintManagerStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintManager_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintManager[] = L"Windows.Graphics.Printing.PrintManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintPageInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintPageInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintPageInfo_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintPageInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintPageInfo[] = L"Windows.Graphics.Printing.PrintPageInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Graphics.Printing.PrintPageRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing.IPrintPageRangeFactory interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintPageRange ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintPageRange_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintPageRange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintPageRange[] = L"Windows.Graphics.Printing.PrintPageRange";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.PrintPageRangeOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintPageRangeOptions ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintPageRangeOptions_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintPageRangeOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintPageRangeOptions[] = L"Windows.Graphics.Printing.PrintPageRangeOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.PrintTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTask ** Default Interface **
 *    Windows.Graphics.Printing.IPrintTaskTargetDeviceSupport
 *    Windows.Graphics.Printing.IPrintTask2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTask_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTask[] = L"Windows.Graphics.Printing.PrintTask";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskCompletedEventArgs[] = L"Windows.Graphics.Printing.PrintTaskCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskOptionsCore ** Default Interface **
 *    Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties
 *    Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration
 *    Windows.Graphics.Printing.IPrintTaskOptions
 *    Windows.Graphics.Printing.IPrintTaskOptions2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskOptions_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskOptions[] = L"Windows.Graphics.Printing.PrintTaskOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskProgressingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskProgressingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskProgressingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskProgressingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskProgressingEventArgs[] = L"Windows.Graphics.Printing.PrintTaskProgressingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskRequest[] = L"Windows.Graphics.Printing.PrintTaskRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskRequestedDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequestedDeferral_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequestedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskRequestedDeferral[] = L"Windows.Graphics.Printing.PrintTaskRequestedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskRequestedEventArgs[] = L"Windows.Graphics.Printing.PrintTaskRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskSourceRequestedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskSourceRequestedArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskSourceRequestedArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskSourceRequestedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskSourceRequestedArgs[] = L"Windows.Graphics.Printing.PrintTaskSourceRequestedArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskSourceRequestedDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskSourceRequestedDeferral_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskSourceRequestedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskSourceRequestedDeferral[] = L"Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.StandardPrintTaskOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic3 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_StandardPrintTaskOptions_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_StandardPrintTaskOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_StandardPrintTaskOptions[] = L"Windows.Graphics.Printing.StandardPrintTaskOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2 __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2 __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2 __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2 __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3 __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        __FIIterator_1_Windows__CGraphics__CPrinting__CPrintPageRange** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting__CPrintPageRange** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* sender,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* sender,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* sender,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintBinding __x_ABI_CWindows_CGraphics_CPrinting_CPrintBinding;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintBordering __x_ABI_CWindows_CGraphics_CPrinting_CPrintBordering;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintCollation __x_ABI_CWindows_CGraphics_CPrinting_CPrintCollation;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintColorMode __x_ABI_CWindows_CGraphics_CPrinting_CPrintColorMode;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintDuplex __x_ABI_CWindows_CGraphics_CPrinting_CPrintDuplex;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintHolePunch __x_ABI_CWindows_CGraphics_CPrinting_CPrintHolePunch;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaSize __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaSize;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaType __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaType;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintOrientation __x_ABI_CWindows_CGraphics_CPrinting_CPrintOrientation;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintQuality __x_ABI_CWindows_CGraphics_CPrinting_CPrintQuality;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintStaple __x_ABI_CWindows_CGraphics_CPrinting_CPrintStaple;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTaskCompletion __x_ABI_CWindows_CGraphics_CPrinting_CPrintTaskCompletion;

typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintPageDescription __x_ABI_CWindows_CGraphics_CPrinting_CPrintPageDescription;

/*
 *
 * Struct Windows.Graphics.Printing.PrintBinding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintBinding
{
    PrintBinding_Default = 0,
    PrintBinding_NotAvailable = 1,
    PrintBinding_PrinterCustom = 2,
    PrintBinding_None = 3,
    PrintBinding_Bale = 4,
    PrintBinding_BindBottom = 5,
    PrintBinding_BindLeft = 6,
    PrintBinding_BindRight = 7,
    PrintBinding_BindTop = 8,
    PrintBinding_Booklet = 9,
    PrintBinding_EdgeStitchBottom = 10,
    PrintBinding_EdgeStitchLeft = 11,
    PrintBinding_EdgeStitchRight = 12,
    PrintBinding_EdgeStitchTop = 13,
    PrintBinding_Fold = 14,
    PrintBinding_JogOffset = 15,
    PrintBinding_Trim = 16,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintBordering
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintBordering
{
    PrintBordering_Default = 0,
    PrintBordering_NotAvailable = 1,
    PrintBordering_PrinterCustom = 2,
    PrintBordering_Bordered = 3,
    PrintBordering_Borderless = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Graphics.Printing.PrintCollation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintCollation
{
    PrintCollation_Default = 0,
    PrintCollation_NotAvailable = 1,
    PrintCollation_PrinterCustom = 2,
    PrintCollation_Collated = 3,
    PrintCollation_Uncollated = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintColorMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintColorMode
{
    PrintColorMode_Default = 0,
    PrintColorMode_NotAvailable = 1,
    PrintColorMode_PrinterCustom = 2,
    PrintColorMode_Color = 3,
    PrintColorMode_Grayscale = 4,
    PrintColorMode_Monochrome = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
    PrintColorMode_AutoSelect = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintDuplex
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintDuplex
{
    PrintDuplex_Default = 0,
    PrintDuplex_NotAvailable = 1,
    PrintDuplex_PrinterCustom = 2,
    PrintDuplex_OneSided = 3,
    PrintDuplex_TwoSidedShortEdge = 4,
    PrintDuplex_TwoSidedLongEdge = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintHolePunch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintHolePunch
{
    PrintHolePunch_Default = 0,
    PrintHolePunch_NotAvailable = 1,
    PrintHolePunch_PrinterCustom = 2,
    PrintHolePunch_None = 3,
    PrintHolePunch_LeftEdge = 4,
    PrintHolePunch_RightEdge = 5,
    PrintHolePunch_TopEdge = 6,
    PrintHolePunch_BottomEdge = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintMediaSize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaSize
{
    PrintMediaSize_Default = 0,
    PrintMediaSize_NotAvailable = 1,
    PrintMediaSize_PrinterCustom = 2,
    PrintMediaSize_BusinessCard = 3,
    PrintMediaSize_CreditCard = 4,
    PrintMediaSize_IsoA0 = 5,
    PrintMediaSize_IsoA1 = 6,
    PrintMediaSize_IsoA10 = 7,
    PrintMediaSize_IsoA2 = 8,
    PrintMediaSize_IsoA3 = 9,
    PrintMediaSize_IsoA3Extra = 10,
    PrintMediaSize_IsoA3Rotated = 11,
    PrintMediaSize_IsoA4 = 12,
    PrintMediaSize_IsoA4Extra = 13,
    PrintMediaSize_IsoA4Rotated = 14,
    PrintMediaSize_IsoA5 = 15,
    PrintMediaSize_IsoA5Extra = 16,
    PrintMediaSize_IsoA5Rotated = 17,
    PrintMediaSize_IsoA6 = 18,
    PrintMediaSize_IsoA6Rotated = 19,
    PrintMediaSize_IsoA7 = 20,
    PrintMediaSize_IsoA8 = 21,
    PrintMediaSize_IsoA9 = 22,
    PrintMediaSize_IsoB0 = 23,
    PrintMediaSize_IsoB1 = 24,
    PrintMediaSize_IsoB10 = 25,
    PrintMediaSize_IsoB2 = 26,
    PrintMediaSize_IsoB3 = 27,
    PrintMediaSize_IsoB4 = 28,
    PrintMediaSize_IsoB4Envelope = 29,
    PrintMediaSize_IsoB5Envelope = 30,
    PrintMediaSize_IsoB5Extra = 31,
    PrintMediaSize_IsoB7 = 32,
    PrintMediaSize_IsoB8 = 33,
    PrintMediaSize_IsoB9 = 34,
    PrintMediaSize_IsoC0 = 35,
    PrintMediaSize_IsoC1 = 36,
    PrintMediaSize_IsoC10 = 37,
    PrintMediaSize_IsoC2 = 38,
    PrintMediaSize_IsoC3 = 39,
    PrintMediaSize_IsoC3Envelope = 40,
    PrintMediaSize_IsoC4 = 41,
    PrintMediaSize_IsoC4Envelope = 42,
    PrintMediaSize_IsoC5 = 43,
    PrintMediaSize_IsoC5Envelope = 44,
    PrintMediaSize_IsoC6 = 45,
    PrintMediaSize_IsoC6C5Envelope = 46,
    PrintMediaSize_IsoC6Envelope = 47,
    PrintMediaSize_IsoC7 = 48,
    PrintMediaSize_IsoC8 = 49,
    PrintMediaSize_IsoC9 = 50,
    PrintMediaSize_IsoDLEnvelope = 51,
    PrintMediaSize_IsoDLEnvelopeRotated = 52,
    PrintMediaSize_IsoSRA3 = 53,
    PrintMediaSize_Japan2LPhoto = 54,
    PrintMediaSize_JapanChou3Envelope = 55,
    PrintMediaSize_JapanChou3EnvelopeRotated = 56,
    PrintMediaSize_JapanChou4Envelope = 57,
    PrintMediaSize_JapanChou4EnvelopeRotated = 58,
    PrintMediaSize_JapanDoubleHagakiPostcard = 59,
    PrintMediaSize_JapanDoubleHagakiPostcardRotated = 60,
    PrintMediaSize_JapanHagakiPostcard = 61,
    PrintMediaSize_JapanHagakiPostcardRotated = 62,
    PrintMediaSize_JapanKaku2Envelope = 63,
    PrintMediaSize_JapanKaku2EnvelopeRotated = 64,
    PrintMediaSize_JapanKaku3Envelope = 65,
    PrintMediaSize_JapanKaku3EnvelopeRotated = 66,
    PrintMediaSize_JapanLPhoto = 67,
    PrintMediaSize_JapanQuadrupleHagakiPostcard = 68,
    PrintMediaSize_JapanYou1Envelope = 69,
    PrintMediaSize_JapanYou2Envelope = 70,
    PrintMediaSize_JapanYou3Envelope = 71,
    PrintMediaSize_JapanYou4Envelope = 72,
    PrintMediaSize_JapanYou4EnvelopeRotated = 73,
    PrintMediaSize_JapanYou6Envelope = 74,
    PrintMediaSize_JapanYou6EnvelopeRotated = 75,
    PrintMediaSize_JisB0 = 76,
    PrintMediaSize_JisB1 = 77,
    PrintMediaSize_JisB10 = 78,
    PrintMediaSize_JisB2 = 79,
    PrintMediaSize_JisB3 = 80,
    PrintMediaSize_JisB4 = 81,
    PrintMediaSize_JisB4Rotated = 82,
    PrintMediaSize_JisB5 = 83,
    PrintMediaSize_JisB5Rotated = 84,
    PrintMediaSize_JisB6 = 85,
    PrintMediaSize_JisB6Rotated = 86,
    PrintMediaSize_JisB7 = 87,
    PrintMediaSize_JisB8 = 88,
    PrintMediaSize_JisB9 = 89,
    PrintMediaSize_NorthAmerica10x11 = 90,
    PrintMediaSize_NorthAmerica10x12 = 91,
    PrintMediaSize_NorthAmerica10x14 = 92,
    PrintMediaSize_NorthAmerica11x17 = 93,
    PrintMediaSize_NorthAmerica14x17 = 94,
    PrintMediaSize_NorthAmerica4x6 = 95,
    PrintMediaSize_NorthAmerica4x8 = 96,
    PrintMediaSize_NorthAmerica5x7 = 97,
    PrintMediaSize_NorthAmerica8x10 = 98,
    PrintMediaSize_NorthAmerica9x11 = 99,
    PrintMediaSize_NorthAmericaArchitectureASheet = 100,
    PrintMediaSize_NorthAmericaArchitectureBSheet = 101,
    PrintMediaSize_NorthAmericaArchitectureCSheet = 102,
    PrintMediaSize_NorthAmericaArchitectureDSheet = 103,
    PrintMediaSize_NorthAmericaArchitectureESheet = 104,
    PrintMediaSize_NorthAmericaCSheet = 105,
    PrintMediaSize_NorthAmericaDSheet = 106,
    PrintMediaSize_NorthAmericaESheet = 107,
    PrintMediaSize_NorthAmericaExecutive = 108,
    PrintMediaSize_NorthAmericaGermanLegalFanfold = 109,
    PrintMediaSize_NorthAmericaGermanStandardFanfold = 110,
    PrintMediaSize_NorthAmericaLegal = 111,
    PrintMediaSize_NorthAmericaLegalExtra = 112,
    PrintMediaSize_NorthAmericaLetter = 113,
    PrintMediaSize_NorthAmericaLetterExtra = 114,
    PrintMediaSize_NorthAmericaLetterPlus = 115,
    PrintMediaSize_NorthAmericaLetterRotated = 116,
    PrintMediaSize_NorthAmericaMonarchEnvelope = 117,
    PrintMediaSize_NorthAmericaNote = 118,
    PrintMediaSize_NorthAmericaNumber10Envelope = 119,
    PrintMediaSize_NorthAmericaNumber10EnvelopeRotated = 120,
    PrintMediaSize_NorthAmericaNumber11Envelope = 121,
    PrintMediaSize_NorthAmericaNumber12Envelope = 122,
    PrintMediaSize_NorthAmericaNumber14Envelope = 123,
    PrintMediaSize_NorthAmericaNumber9Envelope = 124,
    PrintMediaSize_NorthAmericaPersonalEnvelope = 125,
    PrintMediaSize_NorthAmericaQuarto = 126,
    PrintMediaSize_NorthAmericaStatement = 127,
    PrintMediaSize_NorthAmericaSuperA = 128,
    PrintMediaSize_NorthAmericaSuperB = 129,
    PrintMediaSize_NorthAmericaTabloid = 130,
    PrintMediaSize_NorthAmericaTabloidExtra = 131,
    PrintMediaSize_OtherMetricA3Plus = 132,
    PrintMediaSize_OtherMetricA4Plus = 133,
    PrintMediaSize_OtherMetricFolio = 134,
    PrintMediaSize_OtherMetricInviteEnvelope = 135,
    PrintMediaSize_OtherMetricItalianEnvelope = 136,
    PrintMediaSize_Prc10Envelope = 137,
    PrintMediaSize_Prc10EnvelopeRotated = 138,
    PrintMediaSize_Prc16K = 139,
    PrintMediaSize_Prc16KRotated = 140,
    PrintMediaSize_Prc1Envelope = 141,
    PrintMediaSize_Prc1EnvelopeRotated = 142,
    PrintMediaSize_Prc2Envelope = 143,
    PrintMediaSize_Prc2EnvelopeRotated = 144,
    PrintMediaSize_Prc32K = 145,
    PrintMediaSize_Prc32KBig = 146,
    PrintMediaSize_Prc32KRotated = 147,
    PrintMediaSize_Prc3Envelope = 148,
    PrintMediaSize_Prc3EnvelopeRotated = 149,
    PrintMediaSize_Prc4Envelope = 150,
    PrintMediaSize_Prc4EnvelopeRotated = 151,
    PrintMediaSize_Prc5Envelope = 152,
    PrintMediaSize_Prc5EnvelopeRotated = 153,
    PrintMediaSize_Prc6Envelope = 154,
    PrintMediaSize_Prc6EnvelopeRotated = 155,
    PrintMediaSize_Prc7Envelope = 156,
    PrintMediaSize_Prc7EnvelopeRotated = 157,
    PrintMediaSize_Prc8Envelope = 158,
    PrintMediaSize_Prc8EnvelopeRotated = 159,
    PrintMediaSize_Prc9Envelope = 160,
    PrintMediaSize_Prc9EnvelopeRotated = 161,
    PrintMediaSize_Roll04Inch = 162,
    PrintMediaSize_Roll06Inch = 163,
    PrintMediaSize_Roll08Inch = 164,
    PrintMediaSize_Roll12Inch = 165,
    PrintMediaSize_Roll15Inch = 166,
    PrintMediaSize_Roll18Inch = 167,
    PrintMediaSize_Roll22Inch = 168,
    PrintMediaSize_Roll24Inch = 169,
    PrintMediaSize_Roll30Inch = 170,
    PrintMediaSize_Roll36Inch = 171,
    PrintMediaSize_Roll54Inch = 172,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintMediaType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaType
{
    PrintMediaType_Default = 0,
    PrintMediaType_NotAvailable = 1,
    PrintMediaType_PrinterCustom = 2,
    PrintMediaType_AutoSelect = 3,
    PrintMediaType_Archival = 4,
    PrintMediaType_BackPrintFilm = 5,
    PrintMediaType_Bond = 6,
    PrintMediaType_CardStock = 7,
    PrintMediaType_Continuous = 8,
    PrintMediaType_EnvelopePlain = 9,
    PrintMediaType_EnvelopeWindow = 10,
    PrintMediaType_Fabric = 11,
    PrintMediaType_HighResolution = 12,
    PrintMediaType_Label = 13,
    PrintMediaType_MultiLayerForm = 14,
    PrintMediaType_MultiPartForm = 15,
    PrintMediaType_Photographic = 16,
    PrintMediaType_PhotographicFilm = 17,
    PrintMediaType_PhotographicGlossy = 18,
    PrintMediaType_PhotographicHighGloss = 19,
    PrintMediaType_PhotographicMatte = 20,
    PrintMediaType_PhotographicSatin = 21,
    PrintMediaType_PhotographicSemiGloss = 22,
    PrintMediaType_Plain = 23,
    PrintMediaType_Screen = 24,
    PrintMediaType_ScreenPaged = 25,
    PrintMediaType_Stationery = 26,
    PrintMediaType_TabStockFull = 27,
    PrintMediaType_TabStockPreCut = 28,
    PrintMediaType_Transparency = 29,
    PrintMediaType_TShirtTransfer = 30,
    PrintMediaType_None = 31,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintOrientation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintOrientation
{
    PrintOrientation_Default = 0,
    PrintOrientation_NotAvailable = 1,
    PrintOrientation_PrinterCustom = 2,
    PrintOrientation_Portrait = 3,
    PrintOrientation_PortraitFlipped = 4,
    PrintOrientation_Landscape = 5,
    PrintOrientation_LandscapeFlipped = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintQuality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintQuality
{
    PrintQuality_Default = 0,
    PrintQuality_NotAvailable = 1,
    PrintQuality_PrinterCustom = 2,
    PrintQuality_Automatic = 3,
    PrintQuality_Draft = 4,
    PrintQuality_Fax = 5,
    PrintQuality_High = 6,
    PrintQuality_Normal = 7,
    PrintQuality_Photographic = 8,
    PrintQuality_Text = 9,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintStaple
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintStaple
{
    PrintStaple_Default = 0,
    PrintStaple_NotAvailable = 1,
    PrintStaple_PrinterCustom = 2,
    PrintStaple_None = 3,
    PrintStaple_StapleTopLeft = 4,
    PrintStaple_StapleTopRight = 5,
    PrintStaple_StapleBottomLeft = 6,
    PrintStaple_StapleBottomRight = 7,
    PrintStaple_StapleDualLeft = 8,
    PrintStaple_StapleDualRight = 9,
    PrintStaple_StapleDualTop = 10,
    PrintStaple_StapleDualBottom = 11,
    PrintStaple_SaddleStitch = 12,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintTaskCompletion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTaskCompletion
{
    PrintTaskCompletion_Abandoned = 0,
    PrintTaskCompletion_Canceled = 1,
    PrintTaskCompletion_Failed = 2,
    PrintTaskCompletion_Submitted = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.PrintPageDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintPageDescription
{
    struct __x_ABI_CWindows_CFoundation_CSize PageSize;
    struct __x_ABI_CWindows_CFoundation_CRect ImageableRect;
    UINT32 DpiX;
    UINT32 DpiY;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Graphics.Printing.PrintTaskSourceRequestedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* args);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandlerVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_Invoke(This, args) \
    ((This)->lpVtbl->Invoke(This, args))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintDocumentSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintDocumentSource[] = L"Windows.Graphics.Printing.IPrintDocumentSource";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSourceVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintManager[] = L"Windows.Graphics.Printing.IPrintManager";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PrintTaskRequested)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintManager_Windows__CGraphics__CPrinting__CPrintTaskRequestedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_PrintTaskRequested)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_add_PrintTaskRequested(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_PrintTaskRequested(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_remove_PrintTaskRequested(This, eventCookie) \
    ((This)->lpVtbl->remove_PrintTaskRequested(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintManagerStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintManagerStatic[] = L"Windows.Graphics.Printing.IPrintManagerStatic";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStaticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManager** printingManager);
    HRESULT (STDMETHODCALLTYPE* ShowPrintUIAsync)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStaticVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStaticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_GetForCurrentView(This, printingManager) \
    ((This)->lpVtbl->GetForCurrentView(This, printingManager))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_ShowPrintUIAsync(This, operation) \
    ((This)->lpVtbl->ShowPrintUIAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintManagerStatic2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintManagerStatic2[] = L"Windows.Graphics.Printing.IPrintManagerStatic2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintManagerStatic2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintPageInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintPageInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintPageInfo[] = L"Windows.Graphics.Printing.IPrintPageInfo";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_MediaSize)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaSize value);
    HRESULT (STDMETHODCALLTYPE* get_MediaSize)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaSize* value);
    HRESULT (STDMETHODCALLTYPE* put_PageSize)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        struct __x_ABI_CWindows_CFoundation_CSize value);
    HRESULT (STDMETHODCALLTYPE* get_PageSize)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* put_DpiX)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DpiX)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DpiY)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DpiY)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Orientation)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintOrientation value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintOrientation* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfoVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_put_MediaSize(This, value) \
    ((This)->lpVtbl->put_MediaSize(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_get_MediaSize(This, value) \
    ((This)->lpVtbl->get_MediaSize(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_put_PageSize(This, value) \
    ((This)->lpVtbl->put_PageSize(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_get_PageSize(This, value) \
    ((This)->lpVtbl->get_PageSize(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_put_DpiX(This, value) \
    ((This)->lpVtbl->put_DpiX(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_get_DpiX(This, value) \
    ((This)->lpVtbl->get_DpiX(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_put_DpiY(This, value) \
    ((This)->lpVtbl->put_DpiY(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_get_DpiY(This, value) \
    ((This)->lpVtbl->get_DpiY(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_put_Orientation(This, value) \
    ((This)->lpVtbl->put_Orientation(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintPageRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintPageRange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintPageRange[] = L"Windows.Graphics.Printing.IPrintPageRange";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FirstPageNumber)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_LastPageNumber)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_get_FirstPageNumber(This, value) \
    ((This)->lpVtbl->get_FirstPageNumber(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_get_LastPageNumber(This, value) \
    ((This)->lpVtbl->get_LastPageNumber(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintPageRangeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintPageRange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintPageRangeFactory[] = L"Windows.Graphics.Printing.IPrintPageRangeFactory";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory* This,
        INT32 firstPage,
        INT32 lastPage,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithSinglePage)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory* This,
        INT32 page,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRange** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_Create(This, firstPage, lastPage, result) \
    ((This)->lpVtbl->Create(This, firstPage, lastPage, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_CreateWithSinglePage(This, page, result) \
    ((This)->lpVtbl->CreateWithSinglePage(This, page, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintPageRangeOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintPageRangeOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintPageRangeOptions[] = L"Windows.Graphics.Printing.IPrintPageRangeOptions";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_AllowAllPages)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowAllPages)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowCurrentPage)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowCurrentPage)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowCustomSetOfPages)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowCustomSetOfPages)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptionsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_put_AllowAllPages(This, value) \
    ((This)->lpVtbl->put_AllowAllPages(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_get_AllowAllPages(This, value) \
    ((This)->lpVtbl->get_AllowAllPages(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_put_AllowCurrentPage(This, value) \
    ((This)->lpVtbl->put_AllowCurrentPage(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_get_AllowCurrentPage(This, value) \
    ((This)->lpVtbl->get_AllowCurrentPage(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_put_AllowCustomSetOfPages(This, value) \
    ((This)->lpVtbl->put_AllowCustomSetOfPages(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_get_AllowCustomSetOfPages(This, value) \
    ((This)->lpVtbl->get_AllowCustomSetOfPages(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTask[] = L"Windows.Graphics.Printing.IPrintTask";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet** value);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource** value);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore** value);
    HRESULT (STDMETHODCALLTYPE* add_Previewing)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Previewing)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* add_Submitting)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_IInspectable* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Submitting)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* add_Progressing)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskProgressingEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Progressing)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* add_Completed)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintTask_Windows__CGraphics__CPrinting__CPrintTaskCompletedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Completed)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_add_Previewing(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_Previewing(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_remove_Previewing(This, eventCookie) \
    ((This)->lpVtbl->remove_Previewing(This, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_add_Submitting(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_Submitting(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_remove_Submitting(This, eventCookie) \
    ((This)->lpVtbl->remove_Submitting(This, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_add_Progressing(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_Progressing(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_remove_Progressing(This, eventCookie) \
    ((This)->lpVtbl->remove_Progressing(This, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_add_Completed(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_Completed(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_remove_Completed(This, eventCookie) \
    ((This)->lpVtbl->remove_Completed(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTask2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTask2[] = L"Windows.Graphics.Printing.IPrintTask2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_IsPreviewEnabled)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsPreviewEnabled)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_put_IsPreviewEnabled(This, value) \
    ((This)->lpVtbl->put_IsPreviewEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_get_IsPreviewEnabled(This, value) \
    ((This)->lpVtbl->get_IsPreviewEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskCompletedEventArgs[] = L"Windows.Graphics.Printing.IPrintTaskCompletedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Completion)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintTaskCompletion* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_get_Completion(This, value) \
    ((This)->lpVtbl->get_Completion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptions[] = L"Windows.Graphics.Printing.IPrintTaskOptions";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Bordering)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintBordering value);
    HRESULT (STDMETHODCALLTYPE* get_Bordering)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintBordering* value);
    HRESULT (STDMETHODCALLTYPE* GetPagePrintTicket)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageInfo* printPageInfo,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** printTicket);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_put_Bordering(This, value) \
    ((This)->lpVtbl->put_Bordering(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_get_Bordering(This, value) \
    ((This)->lpVtbl->get_Bordering(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_GetPagePrintTicket(This, printPageInfo, printTicket) \
    ((This)->lpVtbl->GetPagePrintTicket(This, printPageInfo, printTicket))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptions2[] = L"Windows.Graphics.Printing.IPrintTaskOptions2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PageRangeOptions)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintPageRangeOptions** value);
    HRESULT (STDMETHODCALLTYPE* get_CustomPageRanges)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2* This,
        __FIVector_1_Windows__CGraphics__CPrinting__CPrintPageRange** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_get_PageRangeOptions(This, value) \
    ((This)->lpVtbl->get_PageRangeOptions(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_get_CustomPageRanges(This, value) \
    ((This)->lpVtbl->get_CustomPageRanges(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptionsCore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptionsCore[] = L"Windows.Graphics.Printing.IPrintTaskOptionsCore";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPageDescription)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore* This,
        UINT32 jobPageNumber,
        struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintPageDescription* description);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_GetPageDescription(This, jobPageNumber, description) \
    ((This)->lpVtbl->GetPageDescription(This, jobPageNumber, description))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptionsCoreProperties[] = L"Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCorePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_MediaSize)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaSize value);
    HRESULT (STDMETHODCALLTYPE* get_MediaSize)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaSize* value);
    HRESULT (STDMETHODCALLTYPE* put_MediaType)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaType value);
    HRESULT (STDMETHODCALLTYPE* get_MediaType)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintMediaType* value);
    HRESULT (STDMETHODCALLTYPE* put_Orientation)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintOrientation value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintOrientation* value);
    HRESULT (STDMETHODCALLTYPE* put_PrintQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintQuality value);
    HRESULT (STDMETHODCALLTYPE* get_PrintQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintQuality* value);
    HRESULT (STDMETHODCALLTYPE* put_ColorMode)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintColorMode value);
    HRESULT (STDMETHODCALLTYPE* get_ColorMode)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintColorMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Duplex)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintDuplex value);
    HRESULT (STDMETHODCALLTYPE* get_Duplex)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintDuplex* value);
    HRESULT (STDMETHODCALLTYPE* put_Collation)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintCollation value);
    HRESULT (STDMETHODCALLTYPE* get_Collation)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintCollation* value);
    HRESULT (STDMETHODCALLTYPE* put_Staple)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintStaple value);
    HRESULT (STDMETHODCALLTYPE* get_Staple)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintStaple* value);
    HRESULT (STDMETHODCALLTYPE* put_HolePunch)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintHolePunch value);
    HRESULT (STDMETHODCALLTYPE* get_HolePunch)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintHolePunch* value);
    HRESULT (STDMETHODCALLTYPE* put_Binding)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintBinding value);
    HRESULT (STDMETHODCALLTYPE* get_Binding)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintBinding* value);
    HRESULT (STDMETHODCALLTYPE* get_MinCopies)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxCopies)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_NumberOfCopies)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfCopies)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCorePropertiesVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCorePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_MediaSize(This, value) \
    ((This)->lpVtbl->put_MediaSize(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_MediaSize(This, value) \
    ((This)->lpVtbl->get_MediaSize(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_MediaType(This, value) \
    ((This)->lpVtbl->put_MediaType(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_MediaType(This, value) \
    ((This)->lpVtbl->get_MediaType(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_Orientation(This, value) \
    ((This)->lpVtbl->put_Orientation(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_PrintQuality(This, value) \
    ((This)->lpVtbl->put_PrintQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_PrintQuality(This, value) \
    ((This)->lpVtbl->get_PrintQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_ColorMode(This, value) \
    ((This)->lpVtbl->put_ColorMode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_ColorMode(This, value) \
    ((This)->lpVtbl->get_ColorMode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_Duplex(This, value) \
    ((This)->lpVtbl->put_Duplex(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_Duplex(This, value) \
    ((This)->lpVtbl->get_Duplex(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_Collation(This, value) \
    ((This)->lpVtbl->put_Collation(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_Collation(This, value) \
    ((This)->lpVtbl->get_Collation(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_Staple(This, value) \
    ((This)->lpVtbl->put_Staple(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_Staple(This, value) \
    ((This)->lpVtbl->get_Staple(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_HolePunch(This, value) \
    ((This)->lpVtbl->put_HolePunch(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_HolePunch(This, value) \
    ((This)->lpVtbl->get_HolePunch(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_Binding(This, value) \
    ((This)->lpVtbl->put_Binding(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_Binding(This, value) \
    ((This)->lpVtbl->get_Binding(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_MinCopies(This, value) \
    ((This)->lpVtbl->get_MinCopies(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_MaxCopies(This, value) \
    ((This)->lpVtbl->get_MaxCopies(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_put_NumberOfCopies(This, value) \
    ((This)->lpVtbl->put_NumberOfCopies(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_get_NumberOfCopies(This, value) \
    ((This)->lpVtbl->get_NumberOfCopies(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskOptionsCoreUIConfiguration[] = L"Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayedOptions)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfigurationVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_get_DisplayedOptions(This, value) \
    ((This)->lpVtbl->get_DisplayedOptions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskProgressingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskProgressingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskProgressingEventArgs[] = L"Windows.Graphics.Printing.IPrintTaskProgressingEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DocumentPageCount)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_get_DocumentPageCount(This, value) \
    ((This)->lpVtbl->get_DocumentPageCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskProgressingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskRequest[] = L"Windows.Graphics.Printing.IPrintTaskRequest";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* CreatePrintTask)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This,
        HSTRING title,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedHandler* handler,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTask** task);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_CreatePrintTask(This, title, handler, task) \
    ((This)->lpVtbl->CreatePrintTask(This, title, handler, task))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskRequestedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskRequestedDeferral[] = L"Windows.Graphics.Printing.IPrintTaskRequestedDeferral";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferralVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskRequestedEventArgs[] = L"Windows.Graphics.Printing.IPrintTaskRequestedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskSourceRequestedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskSourceRequestedArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskSourceRequestedArgs[] = L"Windows.Graphics.Printing.IPrintTaskSourceRequestedArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* SetSource)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource* source);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_SetSource(This, source) \
    ((This)->lpVtbl->SetSource(This, source))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskSourceRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskSourceRequestedDeferral[] = L"Windows.Graphics.Printing.IPrintTaskSourceRequestedDeferral";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferralVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskSourceRequestedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IPrintTaskTargetDeviceSupport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IPrintTaskTargetDeviceSupport[] = L"Windows.Graphics.Printing.IPrintTaskTargetDeviceSupport";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_IsPrinterTargetEnabled)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsPrinterTargetEnabled)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Is3DManufacturingTargetEnabled)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Is3DManufacturingTargetEnabled)(__x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupportVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_put_IsPrinterTargetEnabled(This, value) \
    ((This)->lpVtbl->put_IsPrinterTargetEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_get_IsPrinterTargetEnabled(This, value) \
    ((This)->lpVtbl->get_IsPrinterTargetEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_put_Is3DManufacturingTargetEnabled(This, value) \
    ((This)->lpVtbl->put_Is3DManufacturingTargetEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_get_Is3DManufacturingTargetEnabled(This, value) \
    ((This)->lpVtbl->get_Is3DManufacturingTargetEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskTargetDeviceSupport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.StandardPrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IStandardPrintTaskOptionsStatic[] = L"Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStaticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MediaSize)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MediaType)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PrintQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ColorMode)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Duplex)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Collation)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Staple)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HolePunch)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Binding)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Copies)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NUp)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_InputBin)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStaticVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStaticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_MediaSize(This, value) \
    ((This)->lpVtbl->get_MediaSize(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_MediaType(This, value) \
    ((This)->lpVtbl->get_MediaType(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_PrintQuality(This, value) \
    ((This)->lpVtbl->get_PrintQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_ColorMode(This, value) \
    ((This)->lpVtbl->get_ColorMode(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_Duplex(This, value) \
    ((This)->lpVtbl->get_Duplex(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_Collation(This, value) \
    ((This)->lpVtbl->get_Collation(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_Staple(This, value) \
    ((This)->lpVtbl->get_Staple(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_HolePunch(This, value) \
    ((This)->lpVtbl->get_HolePunch(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_Binding(This, value) \
    ((This)->lpVtbl->get_Binding(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_Copies(This, value) \
    ((This)->lpVtbl->get_Copies(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_NUp(This, value) \
    ((This)->lpVtbl->get_NUp(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_get_InputBin(This, value) \
    ((This)->lpVtbl->get_InputBin(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.StandardPrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IStandardPrintTaskOptionsStatic2[] = L"Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Bordering)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_get_Bordering(This, value) \
    ((This)->lpVtbl->get_Bordering(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.StandardPrintTaskOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_IStandardPrintTaskOptionsStatic3[] = L"Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic3";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CustomPageRanges)(__x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_get_CustomPageRanges(This, value) \
    ((This)->lpVtbl->get_CustomPageRanges(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CIStandardPrintTaskOptionsStatic3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.PrintManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.IPrintManagerStatic2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Printing.IPrintManagerStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintManager_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintManager[] = L"Windows.Graphics.Printing.PrintManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintPageInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintPageInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintPageInfo_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintPageInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintPageInfo[] = L"Windows.Graphics.Printing.PrintPageInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Graphics.Printing.PrintPageRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing.IPrintPageRangeFactory interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintPageRange ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintPageRange_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintPageRange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintPageRange[] = L"Windows.Graphics.Printing.PrintPageRange";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.PrintPageRangeOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintPageRangeOptions ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintPageRangeOptions_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintPageRangeOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintPageRangeOptions[] = L"Windows.Graphics.Printing.PrintPageRangeOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.PrintTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTask ** Default Interface **
 *    Windows.Graphics.Printing.IPrintTaskTargetDeviceSupport
 *    Windows.Graphics.Printing.IPrintTask2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTask_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTask[] = L"Windows.Graphics.Printing.PrintTask";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskCompletedEventArgs[] = L"Windows.Graphics.Printing.PrintTaskCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskOptionsCore ** Default Interface **
 *    Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties
 *    Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration
 *    Windows.Graphics.Printing.IPrintTaskOptions
 *    Windows.Graphics.Printing.IPrintTaskOptions2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskOptions_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskOptions[] = L"Windows.Graphics.Printing.PrintTaskOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskProgressingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskProgressingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskProgressingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskProgressingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskProgressingEventArgs[] = L"Windows.Graphics.Printing.PrintTaskProgressingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskRequest[] = L"Windows.Graphics.Printing.PrintTaskRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskRequestedDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequestedDeferral_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequestedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskRequestedDeferral[] = L"Windows.Graphics.Printing.PrintTaskRequestedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskRequestedEventArgs[] = L"Windows.Graphics.Printing.PrintTaskRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskSourceRequestedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskSourceRequestedArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskSourceRequestedArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskSourceRequestedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskSourceRequestedArgs[] = L"Windows.Graphics.Printing.PrintTaskSourceRequestedArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.IPrintTaskSourceRequestedDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskSourceRequestedDeferral_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintTaskSourceRequestedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintTaskSourceRequestedDeferral[] = L"Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.StandardPrintTaskOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic3 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_StandardPrintTaskOptions_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_StandardPrintTaskOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_StandardPrintTaskOptions[] = L"Windows.Graphics.Printing.StandardPrintTaskOptions";
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
#endif // __windows2Egraphics2Eprinting_p_h__

#endif // __windows2Egraphics2Eprinting_h__
