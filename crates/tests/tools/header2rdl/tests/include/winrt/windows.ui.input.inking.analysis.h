
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
#ifndef __windows2Eui2Einput2Einking2Eanalysis_h__
#define __windows2Eui2Einput2Einking2Eanalysis_h__
#ifndef __windows2Eui2Einput2Einking2Eanalysis_p_h__
#define __windows2Eui2Einput2Einking2Eanalysis_p_h__


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
#include "Windows.UI.Input.Inking.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisInkBullet;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisInkBullet

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisInkDrawing;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisInkDrawing

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisInkWord;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisInkWord

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisLine;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisLine

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisListItem;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisListItem

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisNode;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisParagraph;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisParagraph

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisResult;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisResult

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisRoot;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisRoot

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalysisWritingRegion;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisWritingRegion

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalyzer;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer ABI::Windows::UI::Input::Inking::Analysis::IInkAnalyzer

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        interface IInkAnalyzerFactory;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory ABI::Windows::UI::Input::Inking::Analysis::IInkAnalyzerFactory

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        class InkAnalysisResult;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2c46d1bd-6dbb-5007-ba85-3d0106bddf50"))
IAsyncOperation<ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisResult*, ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Input.Inking.Analysis.InkAnalysisResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisResult*> __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_t;
#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a7ef2666-6fc4-568f-bbf3-19c1036a26bf"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisResult*, ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Input.Inking.Analysis.InkAnalysisResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisResult*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


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



#ifndef DEF___FIIterator_1_UINT32_USE
#define DEF___FIIterator_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f06a2739-9443-5ef0-b284-dc5aff3e7d10"))
IIterator<UINT32> : IIterator_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<UINT32> __FIIterator_1_UINT32_t;
#define __FIIterator_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterator_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_UINT32_USE */



#ifndef DEF___FIIterable_1_UINT32_USE
#define DEF___FIIterable_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("421d4b91-b13b-5f37-ae54-b5249bd80539"))
IIterable<UINT32> : IIterable_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<UINT32> __FIIterable_1_UINT32_t;
#define __FIIterable_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterable_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CPoint_USE
#define DEF___FIIterator_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c602b59e-0a8e-5e99-b478-2b564585278d"))
IIterator<struct ABI::Windows::Foundation::Point> : IIterator_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Foundation::Point> __FIIterator_1_Windows__CFoundation__CPoint_t;
#define __FIIterator_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CPoint_USE
#define DEF___FIIterable_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c192280d-3a09-5423-9dc5-67b83ebde41d"))
IIterable<struct ABI::Windows::Foundation::Point> : IIterable_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Foundation::Point> __FIIterable_1_Windows__CFoundation__CPoint_t;
#define __FIIterable_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ad35ed5c-5f8c-5a68-a6e1-67f209a05ea7"))
IIterator<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*> : IIterator_impl<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Inking.Analysis.IInkAnalysisNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*> __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_t;
#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("784f069e-badd-5258-bd8f-42ce205cc95a"))
IIterable<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*> : IIterable_impl<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Inking.Analysis.IInkAnalysisNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*> __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_t;
#define __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkStroke;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStroke;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke ABI::Windows::UI::Input::Inking::IInkStroke

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5608d5a9-e7e4-5a0b-941f-b7fed76b35bf"))
IIterator<ABI::Windows::UI::Input::Inking::InkStroke*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStroke*, ABI::Windows::UI::Input::Inking::IInkStroke*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Inking.InkStroke>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Inking::InkStroke*> __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_t;
#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bbc11401-89d0-5305-a3b3-36c887714b9b"))
IIterable<ABI::Windows::UI::Input::Inking::InkStroke*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStroke*, ABI::Windows::UI::Input::Inking::IInkStroke*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Inking.InkStroke>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Inking::InkStroke*> __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_t;
#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIVectorView_1_UINT32_USE
#define DEF___FIVectorView_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e5ce1a07-8d33-5007-ba64-7d2508ccf85c"))
IVectorView<UINT32> : IVectorView_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<UINT32> __FIVectorView_1_UINT32_t;
#define __FIVectorView_1_UINT32 ABI::Windows::Foundation::Collections::__FIVectorView_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_UINT32_USE */


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CFoundation__CPoint_USE
#define DEF___FIVectorView_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0b7b4c9d-182f-582a-bddb-42b1aac30cad"))
IVectorView<struct ABI::Windows::Foundation::Point> : IVectorView_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Foundation::Point> __FIVectorView_1_Windows__CFoundation__CPoint_t;
#define __FIVectorView_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2b3fee11-53b5-55b0-8d71-c40b427de029"))
IVectorView<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*> : IVectorView_impl<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.Analysis.IInkAnalysisNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode*> __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_t;
#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        typedef enum InkAnalysisDrawingKind : int InkAnalysisDrawingKind;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        typedef enum InkAnalysisNodeKind : int InkAnalysisNodeKind;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        typedef enum InkAnalysisStatus : int InkAnalysisStatus;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        typedef enum InkAnalysisStrokeKind : int InkAnalysisStrokeKind;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        class InkAnalysisRoot;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        class InkAnalyzer;
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Input.Inking.Analysis.InkAnalysisDrawingKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        enum InkAnalysisDrawingKind : int
                        {
                            InkAnalysisDrawingKind_Drawing = 0,
                            InkAnalysisDrawingKind_Circle = 1,
                            InkAnalysisDrawingKind_Ellipse = 2,
                            InkAnalysisDrawingKind_Triangle = 3,
                            InkAnalysisDrawingKind_IsoscelesTriangle = 4,
                            InkAnalysisDrawingKind_EquilateralTriangle = 5,
                            InkAnalysisDrawingKind_RightTriangle = 6,
                            InkAnalysisDrawingKind_Quadrilateral = 7,
                            InkAnalysisDrawingKind_Rectangle = 8,
                            InkAnalysisDrawingKind_Square = 9,
                            InkAnalysisDrawingKind_Diamond = 10,
                            InkAnalysisDrawingKind_Trapezoid = 11,
                            InkAnalysisDrawingKind_Parallelogram = 12,
                            InkAnalysisDrawingKind_Pentagon = 13,
                            InkAnalysisDrawingKind_Hexagon = 14,
                        };
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.Analysis.InkAnalysisNodeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        enum InkAnalysisNodeKind : int
                        {
                            InkAnalysisNodeKind_UnclassifiedInk = 0,
                            InkAnalysisNodeKind_Root = 1,
                            InkAnalysisNodeKind_WritingRegion = 2,
                            InkAnalysisNodeKind_Paragraph = 3,
                            InkAnalysisNodeKind_Line = 4,
                            InkAnalysisNodeKind_InkWord = 5,
                            InkAnalysisNodeKind_InkBullet = 6,
                            InkAnalysisNodeKind_InkDrawing = 7,
                            InkAnalysisNodeKind_ListItem = 8,
                        };
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.Analysis.InkAnalysisStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        enum InkAnalysisStatus : int
                        {
                            InkAnalysisStatus_Updated = 0,
                            InkAnalysisStatus_Unchanged = 1,
                        };
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.Analysis.InkAnalysisStrokeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        enum InkAnalysisStrokeKind : int
                        {
                            InkAnalysisStrokeKind_Auto = 0,
                            InkAnalysisStrokeKind_Writing = 1,
                            InkAnalysisStrokeKind_Drawing = 2,
                        };
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisInkBullet[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("ee049368-6110-4136-95f9-ee809fc20030")
                        IInkAnalysisInkBullet : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_RecognizedText(
                                HSTRING* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisInkBullet = __uuidof(IInkAnalysisInkBullet);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisInkDrawing[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("6a85ed1f-1fe4-4e15-898c-8e112377e021")
                        IInkAnalysisInkDrawing : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_DrawingKind(
                                ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisDrawingKind* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Center(
                                ABI::Windows::Foundation::Point* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Points(
                                __FIVectorView_1_Windows__CFoundation__CPoint** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisInkDrawing = __uuidof(IInkAnalysisInkDrawing);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisInkWord[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("4bd228ad-83af-4034-8f3b-f8687dfff436")
                        IInkAnalysisInkWord : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_RecognizedText(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_TextAlternates(
                                __FIVectorView_1_HSTRING** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisInkWord = __uuidof(IInkAnalysisInkWord);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisLine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisLine
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisLine[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisLine";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("a06d048d-2b8d-4754-ad5a-d0871193a956")
                        IInkAnalysisLine : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_RecognizedText(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IndentLevel(
                                INT32* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisLine = __uuidof(IInkAnalysisLine);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisListItem
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisListItem[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("b4e3c23f-c4c3-4c3a-a1a6-9d85547ee586")
                        IInkAnalysisListItem : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_RecognizedText(
                                HSTRING* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisListItem = __uuidof(IInkAnalysisListItem);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisNode[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisNode";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("30831f05-5f64-4a2c-ba37-4f4887879574")
                        IInkAnalysisNode : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Id(
                                UINT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Kind(
                                ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisNodeKind* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_BoundingRect(
                                ABI::Windows::Foundation::Rect* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RotatedBoundingRect(
                                __FIVectorView_1_Windows__CFoundation__CPoint** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Children(
                                __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Parent(
                                ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisNode** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetStrokeIds(
                                __FIVectorView_1_UINT32** strokeIds
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisNode = __uuidof(IInkAnalysisNode);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisParagraph[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("d9ad045c-0cd1-4dd4-a68b-eb1f12b3d727")
                        IInkAnalysisParagraph : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_RecognizedText(
                                HSTRING* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisParagraph = __uuidof(IInkAnalysisParagraph);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisResult[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisResult";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("8948ba79-a243-4aa3-a294-1f98bd0ff580")
                        IInkAnalysisResult : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Status(
                                ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisStatus* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisResult = __uuidof(IInkAnalysisResult);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisRoot
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisRoot[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("3fb6a3c4-2fde-4061-8502-a90f32545b84")
                        IInkAnalysisRoot : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_RecognizedText(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindNodes(
                                ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisNodeKind nodeKind,
                                __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisRoot = __uuidof(IInkAnalysisRoot);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisWritingRegion[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("dd6d6231-bd16-4663-b5ae-941d3043ef5b")
                        IInkAnalysisWritingRegion : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_RecognizedText(
                                HSTRING* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalysisWritingRegion = __uuidof(IInkAnalysisWritingRegion);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalyzer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalyzer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalyzer[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalyzer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("f12b8f95-0866-4dc5-8c77-f88614dfe38c")
                        IInkAnalyzer : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_AnalysisRoot(
                                ABI::Windows::UI::Input::Inking::Analysis::IInkAnalysisRoot** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IsAnalyzing(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddDataForStroke(
                                ABI::Windows::UI::Input::Inking::IInkStroke* stroke
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddDataForStrokes(
                                __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* strokes
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ClearDataForAllStrokes(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RemoveDataForStroke(
                                UINT32 strokeId
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RemoveDataForStrokes(
                                __FIIterable_1_UINT32* strokeIds
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ReplaceDataForStroke(
                                ABI::Windows::UI::Input::Inking::IInkStroke* stroke
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetStrokeDataKind(
                                UINT32 strokeId,
                                ABI::Windows::UI::Input::Inking::Analysis::InkAnalysisStrokeKind strokeKind
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AnalyzeAsync(
                                __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalyzer = __uuidof(IInkAnalyzer);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalyzerFactory[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Analysis {
                        MIDL_INTERFACE("29138686-1963-49d8-9589-e14384c769e3")
                        IInkAnalyzerFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateAnalyzer(
                                ABI::Windows::UI::Input::Inking::Analysis::IInkAnalyzer** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInkAnalyzerFactory = __uuidof(IInkAnalyzerFactory);
                    } /* Analysis */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkBullet_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkBullet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisInkBullet[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkDrawing_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkDrawing_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisInkDrawing[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkWord_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkWord_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisInkWord[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisLine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisLine ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisLine_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisLine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisLine[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisLine";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisListItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisListItem_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisListItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisListItem[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisListItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisNode_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisNode[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisParagraph_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisParagraph_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisParagraph[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisResult_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisResult[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisRoot
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisRoot_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisRoot_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisRoot[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisRoot";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisWritingRegion_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisWritingRegion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisWritingRegion[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalyzer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalyzer ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalyzer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalyzer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalyzer[] = L"Windows.UI.Input.Inking.Analysis.InkAnalyzer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult;

typedef struct __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResultVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* This,
        __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#if !defined(____FIIterator_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterator_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterator_1_UINT32 __FIIterator_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_UINT32;

typedef struct __FIIterator_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_UINT32* This,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_UINT32Vtbl;

interface __FIIterator_1_UINT32
{
    CONST_VTBL struct __FIIterator_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_UINT32_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_UINT32_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_UINT32_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_UINT32_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterable_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterable_1_UINT32 __FIIterable_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_UINT32;

typedef struct __FIIterable_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_UINT32* This,
        __FIIterator_1_UINT32** result);

    END_INTERFACE
} __FIIterable_1_UINT32Vtbl;

interface __FIIterable_1_UINT32
{
    CONST_VTBL struct __FIIterable_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_UINT32_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_UINT32_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CPoint __FIIterator_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CPoint;

typedef struct __FIIterator_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CPointVtbl;

interface __FIIterator_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CPoint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CPoint __FIIterable_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CPoint;

typedef struct __FIIterable_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        __FIIterator_1_Windows__CFoundation__CPoint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CPointVtbl;

interface __FIIterable_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CPoint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode;

typedef struct __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode;

typedef struct __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        __FIIterator_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke;

typedef struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke;

typedef struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if !defined(____FIVectorView_1_UINT32_INTERFACE_DEFINED__)
#define ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_UINT32 __FIVectorView_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_UINT32;

typedef struct __FIVectorView_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_UINT32* This,
        UINT32 index,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_UINT32* This,
        UINT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_UINT32* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_UINT32Vtbl;

interface __FIVectorView_1_UINT32
{
    CONST_VTBL struct __FIVectorView_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_UINT32_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_UINT32_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_UINT32_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_UINT32_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CFoundation__CPoint __FIVectorView_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CFoundation__CPoint;

typedef struct __FIVectorView_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        UINT32 index,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CFoundation__CPointVtbl;

interface __FIVectorView_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIVectorView_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CFoundation__CPoint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CFoundation__CPoint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisDrawingKind __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisDrawingKind;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisNodeKind __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisNodeKind;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisStatus __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisStatus;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisStrokeKind __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisStrokeKind;

/*
 *
 * Struct Windows.UI.Input.Inking.Analysis.InkAnalysisDrawingKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisDrawingKind
{
    InkAnalysisDrawingKind_Drawing = 0,
    InkAnalysisDrawingKind_Circle = 1,
    InkAnalysisDrawingKind_Ellipse = 2,
    InkAnalysisDrawingKind_Triangle = 3,
    InkAnalysisDrawingKind_IsoscelesTriangle = 4,
    InkAnalysisDrawingKind_EquilateralTriangle = 5,
    InkAnalysisDrawingKind_RightTriangle = 6,
    InkAnalysisDrawingKind_Quadrilateral = 7,
    InkAnalysisDrawingKind_Rectangle = 8,
    InkAnalysisDrawingKind_Square = 9,
    InkAnalysisDrawingKind_Diamond = 10,
    InkAnalysisDrawingKind_Trapezoid = 11,
    InkAnalysisDrawingKind_Parallelogram = 12,
    InkAnalysisDrawingKind_Pentagon = 13,
    InkAnalysisDrawingKind_Hexagon = 14,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.Analysis.InkAnalysisNodeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisNodeKind
{
    InkAnalysisNodeKind_UnclassifiedInk = 0,
    InkAnalysisNodeKind_Root = 1,
    InkAnalysisNodeKind_WritingRegion = 2,
    InkAnalysisNodeKind_Paragraph = 3,
    InkAnalysisNodeKind_Line = 4,
    InkAnalysisNodeKind_InkWord = 5,
    InkAnalysisNodeKind_InkBullet = 6,
    InkAnalysisNodeKind_InkDrawing = 7,
    InkAnalysisNodeKind_ListItem = 8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.Analysis.InkAnalysisStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisStatus
{
    InkAnalysisStatus_Updated = 0,
    InkAnalysisStatus_Unchanged = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.Analysis.InkAnalysisStrokeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisStrokeKind
{
    InkAnalysisStrokeKind_Auto = 0,
    InkAnalysisStrokeKind_Writing = 1,
    InkAnalysisStrokeKind_Drawing = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisInkBullet[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBulletVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecognizedText)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBulletVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBulletVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_get_RecognizedText(This, value) \
    ((This)->lpVtbl->get_RecognizedText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkBullet_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisInkDrawing[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DrawingKind)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisDrawingKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Center)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Points)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing* This,
        __FIVectorView_1_Windows__CFoundation__CPoint** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawingVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_get_DrawingKind(This, value) \
    ((This)->lpVtbl->get_DrawingKind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_get_Center(This, value) \
    ((This)->lpVtbl->get_Center(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_get_Points(This, value) \
    ((This)->lpVtbl->get_Points(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkDrawing_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisInkWord[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecognizedText)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TextAlternates)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWordVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_get_RecognizedText(This, value) \
    ((This)->lpVtbl->get_RecognizedText(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_get_TextAlternates(This, value) \
    ((This)->lpVtbl->get_TextAlternates(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisInkWord_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisLine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisLine
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisLine[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisLine";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecognizedText)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IndentLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLineVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_get_RecognizedText(This, value) \
    ((This)->lpVtbl->get_RecognizedText(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_get_IndentLevel(This, value) \
    ((This)->lpVtbl->get_IndentLevel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisLine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisListItem
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisListItem[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecognizedText)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItemVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_get_RecognizedText(This, value) \
    ((This)->lpVtbl->get_RecognizedText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisListItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisNode[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisNode";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisNodeKind* value);
    HRESULT (STDMETHODCALLTYPE* get_BoundingRect)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_RotatedBoundingRect)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        __FIVectorView_1_Windows__CFoundation__CPoint** value);
    HRESULT (STDMETHODCALLTYPE* get_Children)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode** value);
    HRESULT (STDMETHODCALLTYPE* get_Parent)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode** value);
    HRESULT (STDMETHODCALLTYPE* GetStrokeIds)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode* This,
        __FIVectorView_1_UINT32** strokeIds);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNodeVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_get_BoundingRect(This, value) \
    ((This)->lpVtbl->get_BoundingRect(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_get_RotatedBoundingRect(This, value) \
    ((This)->lpVtbl->get_RotatedBoundingRect(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_get_Children(This, value) \
    ((This)->lpVtbl->get_Children(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_get_Parent(This, value) \
    ((This)->lpVtbl->get_Parent(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_GetStrokeIds(This, strokeIds) \
    ((This)->lpVtbl->GetStrokeIds(This, strokeIds))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisParagraph[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraphVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecognizedText)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraphVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraphVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_get_RecognizedText(This, value) \
    ((This)->lpVtbl->get_RecognizedText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisParagraph_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisResult[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisResult";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResultVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisRoot
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisRoot[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRootVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecognizedText)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* FindNodes)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisNodeKind nodeKind,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CAnalysis__CIInkAnalysisNode** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRootVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRootVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_get_RecognizedText(This, value) \
    ((This)->lpVtbl->get_RecognizedText(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_FindNodes(This, nodeKind, result) \
    ((This)->lpVtbl->FindNodes(This, nodeKind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalysisWritingRegion[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecognizedText)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegionVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_get_RecognizedText(This, value) \
    ((This)->lpVtbl->get_RecognizedText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisWritingRegion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalyzer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Analysis.InkAnalyzer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalyzer[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalyzer";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AnalysisRoot)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalysisRoot** value);
    HRESULT (STDMETHODCALLTYPE* get_IsAnalyzing)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* AddDataForStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* stroke);
    HRESULT (STDMETHODCALLTYPE* AddDataForStrokes)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* strokes);
    HRESULT (STDMETHODCALLTYPE* ClearDataForAllStrokes)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This);
    HRESULT (STDMETHODCALLTYPE* RemoveDataForStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        UINT32 strokeId);
    HRESULT (STDMETHODCALLTYPE* RemoveDataForStrokes)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        __FIIterable_1_UINT32* strokeIds);
    HRESULT (STDMETHODCALLTYPE* ReplaceDataForStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* stroke);
    HRESULT (STDMETHODCALLTYPE* SetStrokeDataKind)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        UINT32 strokeId,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CInkAnalysisStrokeKind strokeKind);
    HRESULT (STDMETHODCALLTYPE* AnalyzeAsync)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer* This,
        __FIAsyncOperation_1_Windows__CUI__CInput__CInking__CAnalysis__CInkAnalysisResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_get_AnalysisRoot(This, value) \
    ((This)->lpVtbl->get_AnalysisRoot(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_get_IsAnalyzing(This, value) \
    ((This)->lpVtbl->get_IsAnalyzing(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_AddDataForStroke(This, stroke) \
    ((This)->lpVtbl->AddDataForStroke(This, stroke))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_AddDataForStrokes(This, strokes) \
    ((This)->lpVtbl->AddDataForStrokes(This, strokes))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_ClearDataForAllStrokes(This) \
    ((This)->lpVtbl->ClearDataForAllStrokes(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_RemoveDataForStroke(This, strokeId) \
    ((This)->lpVtbl->RemoveDataForStroke(This, strokeId))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_RemoveDataForStrokes(This, strokeIds) \
    ((This)->lpVtbl->RemoveDataForStrokes(This, strokeIds))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_ReplaceDataForStroke(This, stroke) \
    ((This)->lpVtbl->ReplaceDataForStroke(This, stroke))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_SetStrokeDataKind(This, strokeId, strokeKind) \
    ((This)->lpVtbl->SetStrokeDataKind(This, strokeId, strokeKind))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_AnalyzeAsync(This, result) \
    ((This)->lpVtbl->AnalyzeAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Analysis_IInkAnalyzerFactory[] = L"Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAnalyzer)(__x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzer** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactoryVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_CreateAnalyzer(This, result) \
    ((This)->lpVtbl->CreateAnalyzer(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CAnalysis_CIInkAnalyzerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisInkBullet ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkBullet_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkBullet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisInkBullet[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisInkDrawing ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkDrawing_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkDrawing_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisInkDrawing[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisInkWord ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkWord_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisInkWord_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisInkWord[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisLine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisLine ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisLine_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisLine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisLine[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisLine";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisListItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisListItem ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisListItem_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisListItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisListItem[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisListItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisNode_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisNode[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisParagraph ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisParagraph_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisParagraph_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisParagraph[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisResult_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisResult[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisRoot
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisRoot ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisRoot_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisRoot_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisRoot[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisRoot";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisWritingRegion ** Default Interface **
 *    Windows.UI.Input.Inking.Analysis.IInkAnalysisNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisWritingRegion_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalysisWritingRegion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalysisWritingRegion[] = L"Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.Analysis.InkAnalyzer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Analysis.IInkAnalyzer ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalyzer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Analysis_InkAnalyzer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Analysis_InkAnalyzer[] = L"Windows.UI.Input.Inking.Analysis.InkAnalyzer";
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
#endif // __windows2Eui2Einput2Einking2Eanalysis_p_h__

#endif // __windows2Eui2Einput2Einking2Eanalysis_h__
