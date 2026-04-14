
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
#ifndef __windows2Edata2Etext_h__
#define __windows2Edata2Etext_h__
#ifndef __windows2Edata2Etext_p_h__
#define __windows2Edata2Etext_p_h__


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
#include "Windows.UI.Text.Core.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ISelectableWordSegmentsTokenizingHandler;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler ABI::Windows::Data::Text::ISelectableWordSegmentsTokenizingHandler

#endif // ____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface IWordSegmentsTokenizingHandler;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler ABI::Windows::Data::Text::IWordSegmentsTokenizingHandler

#endif // ____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIAlternateWordForm_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIAlternateWordForm_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface IAlternateWordForm;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm ABI::Windows::Data::Text::IAlternateWordForm

#endif // ____x_ABI_CWindows_CData_CText_CIAlternateWordForm_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISelectableWordSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISelectableWordSegment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ISelectableWordSegment;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment ABI::Windows::Data::Text::ISelectableWordSegment

#endif // ____x_ABI_CWindows_CData_CText_CISelectableWordSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ISelectableWordsSegmenter;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter ABI::Windows::Data::Text::ISelectableWordsSegmenter

#endif // ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ISelectableWordsSegmenterFactory;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory ABI::Windows::Data::Text::ISelectableWordsSegmenterFactory

#endif // ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISemanticTextQuery_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISemanticTextQuery_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ISemanticTextQuery;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery ABI::Windows::Data::Text::ISemanticTextQuery

#endif // ____x_ABI_CWindows_CData_CText_CISemanticTextQuery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ISemanticTextQueryFactory;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory ABI::Windows::Data::Text::ISemanticTextQueryFactory

#endif // ____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextConversionGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextConversionGenerator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextConversionGenerator;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator ABI::Windows::Data::Text::ITextConversionGenerator

#endif // ____x_ABI_CWindows_CData_CText_CITextConversionGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextConversionGeneratorFactory;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory ABI::Windows::Data::Text::ITextConversionGeneratorFactory

#endif // ____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextPhoneme_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextPhoneme_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextPhoneme;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextPhoneme ABI::Windows::Data::Text::ITextPhoneme

#endif // ____x_ABI_CWindows_CData_CText_CITextPhoneme_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextPredictionGenerator;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator ABI::Windows::Data::Text::ITextPredictionGenerator

#endif // ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextPredictionGenerator2;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2 ABI::Windows::Data::Text::ITextPredictionGenerator2

#endif // ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextPredictionGeneratorFactory;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory ABI::Windows::Data::Text::ITextPredictionGeneratorFactory

#endif // ____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextReverseConversionGenerator;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator ABI::Windows::Data::Text::ITextReverseConversionGenerator

#endif // ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextReverseConversionGenerator2;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2 ABI::Windows::Data::Text::ITextReverseConversionGenerator2

#endif // ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface ITextReverseConversionGeneratorFactory;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory ABI::Windows::Data::Text::ITextReverseConversionGeneratorFactory

#endif // ____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface IUnicodeCharactersStatics;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics ABI::Windows::Data::Text::IUnicodeCharactersStatics

#endif // ____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIWordSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIWordSegment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface IWordSegment;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CIWordSegment ABI::Windows::Data::Text::IWordSegment

#endif // ____x_ABI_CWindows_CData_CText_CIWordSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIWordsSegmenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIWordsSegmenter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface IWordsSegmenter;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter ABI::Windows::Data::Text::IWordsSegmenter

#endif // ____x_ABI_CWindows_CData_CText_CIWordsSegmenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                interface IWordsSegmenterFactory;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory ABI::Windows::Data::Text::IWordsSegmenterFactory

#endif // ____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */



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



#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f92b529-119b-575a-a419-3904b4e41af2"))
IAsyncOperation<__FIVectorView_1_HSTRING*> : IAsyncOperation_impl<__FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_HSTRING*> __FIAsyncOperation_1___FIVectorView_1_HSTRING_t;
#define __FIAsyncOperation_1___FIVectorView_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7c7899be-5f2e-5bf3-ade5-ad98b772c7cd"))
IAsyncOperationCompletedHandler<__FIVectorView_1_HSTRING*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_HSTRING*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class TextPhoneme;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CData__CText__CTextPhoneme_USE
#define DEF___FIIterator_1_Windows__CData__CText__CTextPhoneme_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("36426c36-b5c4-5d1b-a468-aff3b48b7c3a"))
IIterator<ABI::Windows::Data::Text::TextPhoneme*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::TextPhoneme*, ABI::Windows::Data::Text::ITextPhoneme*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Data.Text.TextPhoneme>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Data::Text::TextPhoneme*> __FIIterator_1_Windows__CData__CText__CTextPhoneme_t;
#define __FIIterator_1_Windows__CData__CText__CTextPhoneme ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CData__CText__CTextPhoneme_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CData__CText__CTextPhoneme_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CData__CText__CTextPhoneme_USE
#define DEF___FIIterable_1_Windows__CData__CText__CTextPhoneme_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2017f561-8162-55f8-b056-656636935815"))
IIterable<ABI::Windows::Data::Text::TextPhoneme*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::TextPhoneme*, ABI::Windows::Data::Text::ITextPhoneme*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Data.Text.TextPhoneme>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Data::Text::TextPhoneme*> __FIIterable_1_Windows__CData__CText__CTextPhoneme_t;
#define __FIIterable_1_Windows__CData__CText__CTextPhoneme ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CData__CText__CTextPhoneme_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CData__CText__CTextPhoneme_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE
#define DEF___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cfc6e66d-5e2a-582d-8b6d-fbf71cf3d0eb"))
IVectorView<ABI::Windows::Data::Text::TextPhoneme*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::TextPhoneme*, ABI::Windows::Data::Text::ITextPhoneme*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Data.Text.TextPhoneme>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Data::Text::TextPhoneme*> __FIVectorView_1_Windows__CData__CText__CTextPhoneme_t;
#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CData__CText__CTextPhoneme_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6bc3019d-dd10-5510-b164-808c232b7d64"))
IAsyncOperation<__FIVectorView_1_Windows__CData__CText__CTextPhoneme*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CData__CText__CTextPhoneme*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Data.Text.TextPhoneme>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CData__CText__CTextPhoneme*> __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("83e14307-0be1-5560-8bfc-291095cf6d30"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CData__CText__CTextPhoneme*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CData__CText__CTextPhoneme*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Data.Text.TextPhoneme>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CData__CText__CTextPhoneme*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class AlternateWordForm;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CData__CText__CAlternateWordForm_USE
#define DEF___FIIterator_1_Windows__CData__CText__CAlternateWordForm_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7f463f8a-e08f-516e-b4b8-81b5417bfb58"))
IIterator<ABI::Windows::Data::Text::AlternateWordForm*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::AlternateWordForm*, ABI::Windows::Data::Text::IAlternateWordForm*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Data.Text.AlternateWordForm>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Data::Text::AlternateWordForm*> __FIIterator_1_Windows__CData__CText__CAlternateWordForm_t;
#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CData__CText__CAlternateWordForm_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CData__CText__CAlternateWordForm_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CData__CText__CAlternateWordForm_USE
#define DEF___FIIterable_1_Windows__CData__CText__CAlternateWordForm_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ae838c29-60c1-5093-ae61-0ed22857db05"))
IIterable<ABI::Windows::Data::Text::AlternateWordForm*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::AlternateWordForm*, ABI::Windows::Data::Text::IAlternateWordForm*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Data.Text.AlternateWordForm>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Data::Text::AlternateWordForm*> __FIIterable_1_Windows__CData__CText__CAlternateWordForm_t;
#define __FIIterable_1_Windows__CData__CText__CAlternateWordForm ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CData__CText__CAlternateWordForm_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CData__CText__CAlternateWordForm_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class SelectableWordSegment;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CData__CText__CSelectableWordSegment_USE
#define DEF___FIIterator_1_Windows__CData__CText__CSelectableWordSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a5842459-147d-5a88-8f23-a3398afd8a85"))
IIterator<ABI::Windows::Data::Text::SelectableWordSegment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::SelectableWordSegment*, ABI::Windows::Data::Text::ISelectableWordSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Data.Text.SelectableWordSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Data::Text::SelectableWordSegment*> __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_t;
#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CData__CText__CSelectableWordSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CData__CText__CSelectableWordSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CData__CText__CSelectableWordSegment_USE
#define DEF___FIIterable_1_Windows__CData__CText__CSelectableWordSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("784b0cec-1348-5334-91e5-2e013294e211"))
IIterable<ABI::Windows::Data::Text::SelectableWordSegment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::SelectableWordSegment*, ABI::Windows::Data::Text::ISelectableWordSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Data.Text.SelectableWordSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Data::Text::SelectableWordSegment*> __FIIterable_1_Windows__CData__CText__CSelectableWordSegment_t;
#define __FIIterable_1_Windows__CData__CText__CSelectableWordSegment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CData__CText__CSelectableWordSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CData__CText__CSelectableWordSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                typedef struct TextSegment TextSegment;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CData__CText__CTextSegment_USE
#define DEF___FIIterator_1_Windows__CData__CText__CTextSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("645a39b4-f001-5272-9015-fb4a327179ae"))
IIterator<struct ABI::Windows::Data::Text::TextSegment> : IIterator_impl<struct ABI::Windows::Data::Text::TextSegment>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Data.Text.TextSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Data::Text::TextSegment> __FIIterator_1_Windows__CData__CText__CTextSegment_t;
#define __FIIterator_1_Windows__CData__CText__CTextSegment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CData__CText__CTextSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CData__CText__CTextSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CData__CText__CTextSegment_USE
#define DEF___FIIterable_1_Windows__CData__CText__CTextSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5498f4f3-cee4-5b72-9729-815c4ad7b9dc"))
IIterable<struct ABI::Windows::Data::Text::TextSegment> : IIterable_impl<struct ABI::Windows::Data::Text::TextSegment>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Data.Text.TextSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Data::Text::TextSegment> __FIIterable_1_Windows__CData__CText__CTextSegment_t;
#define __FIIterable_1_Windows__CData__CText__CTextSegment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CData__CText__CTextSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CData__CText__CTextSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class WordSegment;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CData__CText__CWordSegment_USE
#define DEF___FIIterator_1_Windows__CData__CText__CWordSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aeb846b3-664b-545e-ab93-f5fc66d24e32"))
IIterator<ABI::Windows::Data::Text::WordSegment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::WordSegment*, ABI::Windows::Data::Text::IWordSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Data.Text.WordSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Data::Text::WordSegment*> __FIIterator_1_Windows__CData__CText__CWordSegment_t;
#define __FIIterator_1_Windows__CData__CText__CWordSegment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CData__CText__CWordSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CData__CText__CWordSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CData__CText__CWordSegment_USE
#define DEF___FIIterable_1_Windows__CData__CText__CWordSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d14dc94a-a311-5ddc-bf8b-d58795d87d6d"))
IIterable<ABI::Windows::Data::Text::WordSegment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::WordSegment*, ABI::Windows::Data::Text::IWordSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Data.Text.WordSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Data::Text::WordSegment*> __FIIterable_1_Windows__CData__CText__CWordSegment_t;
#define __FIIterable_1_Windows__CData__CText__CWordSegment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CData__CText__CWordSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CData__CText__CWordSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CData__CText__CAlternateWordForm_USE
#define DEF___FIVectorView_1_Windows__CData__CText__CAlternateWordForm_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6b742ff2-746a-5545-a6ed-3bba453cf5d9"))
IVectorView<ABI::Windows::Data::Text::AlternateWordForm*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::AlternateWordForm*, ABI::Windows::Data::Text::IAlternateWordForm*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Data.Text.AlternateWordForm>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Data::Text::AlternateWordForm*> __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_t;
#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CData__CText__CAlternateWordForm_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CData__CText__CAlternateWordForm_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_USE
#define DEF___FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("33f90a72-86f4-5027-b50a-6939a1f9d560"))
IVectorView<ABI::Windows::Data::Text::SelectableWordSegment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::SelectableWordSegment*, ABI::Windows::Data::Text::ISelectableWordSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Data.Text.SelectableWordSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Data::Text::SelectableWordSegment*> __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_t;
#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CData__CText__CTextSegment_USE
#define DEF___FIVectorView_1_Windows__CData__CText__CTextSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f245f9d-eb5f-5641-9dcc-6ab1946cc7e6"))
IVectorView<struct ABI::Windows::Data::Text::TextSegment> : IVectorView_impl<struct ABI::Windows::Data::Text::TextSegment>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Data.Text.TextSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Data::Text::TextSegment> __FIVectorView_1_Windows__CData__CText__CTextSegment_t;
#define __FIVectorView_1_Windows__CData__CText__CTextSegment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CData__CText__CTextSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CData__CText__CTextSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CData__CText__CWordSegment_USE
#define DEF___FIVectorView_1_Windows__CData__CText__CWordSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c706749a-e11d-5e07-8534-2bd23ec210f9"))
IVectorView<ABI::Windows::Data::Text::WordSegment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Text::WordSegment*, ABI::Windows::Data::Text::IWordSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Data.Text.WordSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Data::Text::WordSegment*> __FIVectorView_1_Windows__CData__CText__CWordSegment_t;
#define __FIVectorView_1_Windows__CData__CText__CWordSegment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CData__CText__CWordSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CData__CText__CWordSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    typedef enum CoreTextInputScope : int CoreTextInputScope;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                typedef enum AlternateNormalizationFormat : int AlternateNormalizationFormat;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                typedef enum TextPredictionOptions : unsigned int TextPredictionOptions;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                typedef enum UnicodeGeneralCategory : int UnicodeGeneralCategory;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                typedef enum UnicodeNumericType : int UnicodeNumericType;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class SelectableWordsSegmenter;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class SemanticTextQuery;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class TextConversionGenerator;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class TextPredictionGenerator;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class TextReverseConversionGenerator;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                class WordsSegmenter;
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Data.Text.AlternateNormalizationFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                enum AlternateNormalizationFormat : int
                {
                    AlternateNormalizationFormat_NotNormalized = 0,
                    AlternateNormalizationFormat_Number = 1,
                    AlternateNormalizationFormat_Currency = 3,
                    AlternateNormalizationFormat_Date = 4,
                    AlternateNormalizationFormat_Time = 5,
                };
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Data.Text.TextPredictionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                enum TextPredictionOptions : unsigned int
                {
                    TextPredictionOptions_None = 0,
                    TextPredictionOptions_Predictions = 0x1,
                    TextPredictionOptions_Corrections = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(TextPredictionOptions)
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Data.Text.UnicodeGeneralCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                enum UnicodeGeneralCategory : int
                {
                    UnicodeGeneralCategory_UppercaseLetter = 0,
                    UnicodeGeneralCategory_LowercaseLetter = 1,
                    UnicodeGeneralCategory_TitlecaseLetter = 2,
                    UnicodeGeneralCategory_ModifierLetter = 3,
                    UnicodeGeneralCategory_OtherLetter = 4,
                    UnicodeGeneralCategory_NonspacingMark = 5,
                    UnicodeGeneralCategory_SpacingCombiningMark = 6,
                    UnicodeGeneralCategory_EnclosingMark = 7,
                    UnicodeGeneralCategory_DecimalDigitNumber = 8,
                    UnicodeGeneralCategory_LetterNumber = 9,
                    UnicodeGeneralCategory_OtherNumber = 10,
                    UnicodeGeneralCategory_SpaceSeparator = 11,
                    UnicodeGeneralCategory_LineSeparator = 12,
                    UnicodeGeneralCategory_ParagraphSeparator = 13,
                    UnicodeGeneralCategory_Control = 14,
                    UnicodeGeneralCategory_Format = 15,
                    UnicodeGeneralCategory_Surrogate = 16,
                    UnicodeGeneralCategory_PrivateUse = 17,
                    UnicodeGeneralCategory_ConnectorPunctuation = 18,
                    UnicodeGeneralCategory_DashPunctuation = 19,
                    UnicodeGeneralCategory_OpenPunctuation = 20,
                    UnicodeGeneralCategory_ClosePunctuation = 21,
                    UnicodeGeneralCategory_InitialQuotePunctuation = 22,
                    UnicodeGeneralCategory_FinalQuotePunctuation = 23,
                    UnicodeGeneralCategory_OtherPunctuation = 24,
                    UnicodeGeneralCategory_MathSymbol = 25,
                    UnicodeGeneralCategory_CurrencySymbol = 26,
                    UnicodeGeneralCategory_ModifierSymbol = 27,
                    UnicodeGeneralCategory_OtherSymbol = 28,
                    UnicodeGeneralCategory_NotAssigned = 29,
                };
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Data.Text.UnicodeNumericType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                enum UnicodeNumericType : int
                {
                    UnicodeNumericType_None = 0,
                    UnicodeNumericType_Decimal = 1,
                    UnicodeNumericType_Digit = 2,
                    UnicodeNumericType_Numeric = 3,
                };
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Data.Text.TextSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                struct TextSegment
                {
                    UINT32 StartPosition;
                    UINT32 Length;
                };
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Data.Text.SelectableWordSegmentsTokenizingHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("3a3dfc9c-aede-4dc7-9e6c-41c044bd3592")
                ISelectableWordSegmentsTokenizingHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        __FIIterable_1_Windows__CData__CText__CSelectableWordSegment* precedingWords,
                        __FIIterable_1_Windows__CData__CText__CSelectableWordSegment* words
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISelectableWordSegmentsTokenizingHandler = __uuidof(ISelectableWordSegmentsTokenizingHandler);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Data.Text.WordSegmentsTokenizingHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("a5dd6357-bf2a-4c4f-a31f-29e71c6f8b35")
                IWordSegmentsTokenizingHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        __FIIterable_1_Windows__CData__CText__CWordSegment* precedingWords,
                        __FIIterable_1_Windows__CData__CText__CWordSegment* words
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWordSegmentsTokenizingHandler = __uuidof(IWordSegmentsTokenizingHandler);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IAlternateWordForm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.AlternateWordForm
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIAlternateWordForm_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIAlternateWordForm_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IAlternateWordForm[] = L"Windows.Data.Text.IAlternateWordForm";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("47396c1e-51b9-4207-9146-248e636a1d1d")
                IAlternateWordForm : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SourceTextSegment(
                        ABI::Windows::Data::Text::TextSegment* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlternateText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NormalizationFormat(
                        ABI::Windows::Data::Text::AlternateNormalizationFormat* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAlternateWordForm = __uuidof(IAlternateWordForm);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIAlternateWordForm;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIAlternateWordForm_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISelectableWordSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SelectableWordSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISelectableWordSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISelectableWordSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISelectableWordSegment[] = L"Windows.Data.Text.ISelectableWordSegment";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("916a4cb7-8aa7-4c78-b374-5dedb752e60b")
                ISelectableWordSegment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceTextSegment(
                        ABI::Windows::Data::Text::TextSegment* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISelectableWordSegment = __uuidof(ISelectableWordSegment);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISelectableWordSegment;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISelectableWordSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISelectableWordsSegmenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SelectableWordsSegmenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISelectableWordsSegmenter[] = L"Windows.Data.Text.ISelectableWordsSegmenter";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("f6dc31e7-4b13-45c5-8897-7d71269e085d")
                ISelectableWordsSegmenter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTokenAt(
                        HSTRING text,
                        UINT32 startIndex,
                        ABI::Windows::Data::Text::ISelectableWordSegment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTokens(
                        HSTRING text,
                        __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Tokenize(
                        HSTRING text,
                        UINT32 startIndex,
                        ABI::Windows::Data::Text::ISelectableWordSegmentsTokenizingHandler* handler
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISelectableWordsSegmenter = __uuidof(ISelectableWordsSegmenter);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISelectableWordsSegmenterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SelectableWordsSegmenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISelectableWordsSegmenterFactory[] = L"Windows.Data.Text.ISelectableWordsSegmenterFactory";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("8c7a7648-6057-4339-bc70-f210010a4150")
                ISelectableWordsSegmenterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithLanguage(
                        HSTRING language,
                        ABI::Windows::Data::Text::ISelectableWordsSegmenter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISelectableWordsSegmenterFactory = __uuidof(ISelectableWordsSegmenterFactory);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISemanticTextQuery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SemanticTextQuery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISemanticTextQuery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISemanticTextQuery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISemanticTextQuery[] = L"Windows.Data.Text.ISemanticTextQuery";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("6a1cab51-1fb2-4909-80b8-35731a2b3e7f")
                ISemanticTextQuery : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Find(
                        HSTRING content,
                        __FIVectorView_1_Windows__CData__CText__CTextSegment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindInProperty(
                        HSTRING propertyContent,
                        HSTRING propertyName,
                        __FIVectorView_1_Windows__CData__CText__CTextSegment** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISemanticTextQuery = __uuidof(ISemanticTextQuery);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISemanticTextQuery;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISemanticTextQuery_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISemanticTextQueryFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SemanticTextQuery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISemanticTextQueryFactory[] = L"Windows.Data.Text.ISemanticTextQueryFactory";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("238c0503-f995-4587-8777-a2b7d80acfef")
                ISemanticTextQueryFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING aqsFilter,
                        ABI::Windows::Data::Text::ISemanticTextQuery** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithLanguage(
                        HSTRING aqsFilter,
                        HSTRING filterLanguage,
                        ABI::Windows::Data::Text::ISemanticTextQuery** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISemanticTextQueryFactory = __uuidof(ISemanticTextQueryFactory);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextConversionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextConversionGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextConversionGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextConversionGenerator[] = L"Windows.Data.Text.ITextConversionGenerator";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("03606a5e-2aa9-4ab6-af8b-a562b63a8992")
                ITextConversionGenerator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LanguageAvailableButNotInstalled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCandidatesAsync(
                        HSTRING input,
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCandidatesWithMaxCountAsync(
                        HSTRING input,
                        UINT32 maxCandidates,
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextConversionGenerator = __uuidof(ITextConversionGenerator);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextConversionGenerator;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextConversionGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextConversionGeneratorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextConversionGeneratorFactory[] = L"Windows.Data.Text.ITextConversionGeneratorFactory";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("fcaa3781-3083-49ab-be15-56dfbbb74d6f")
                ITextConversionGeneratorFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING languageTag,
                        ABI::Windows::Data::Text::ITextConversionGenerator** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextConversionGeneratorFactory = __uuidof(ITextConversionGeneratorFactory);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextPhoneme
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextPhoneme
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CData_CText_CITextPhoneme_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextPhoneme_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextPhoneme[] = L"Windows.Data.Text.ITextPhoneme";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("9362a40a-9b7a-4569-94cf-d84f2f38cf9b")
                ITextPhoneme : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReadingText(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextPhoneme = __uuidof(ITextPhoneme);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextPhoneme;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextPhoneme_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Data.Text.ITextPredictionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextPredictionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextPredictionGenerator[] = L"Windows.Data.Text.ITextPredictionGenerator";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("5eacab07-abf1-4cb6-9d9e-326f2b468756")
                ITextPredictionGenerator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LanguageAvailableButNotInstalled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCandidatesAsync(
                        HSTRING input,
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCandidatesWithMaxCountAsync(
                        HSTRING input,
                        UINT32 maxCandidates,
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextPredictionGenerator = __uuidof(ITextPredictionGenerator);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextPredictionGenerator;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextPredictionGenerator2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextPredictionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextPredictionGenerator2[] = L"Windows.Data.Text.ITextPredictionGenerator2";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("b84723b8-2c77-486a-900a-a3453eedc15d")
                ITextPredictionGenerator2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCandidatesWithParametersAsync(
                        HSTRING input,
                        UINT32 maxCandidates,
                        ABI::Windows::Data::Text::TextPredictionOptions predictionOptions,
                        __FIIterable_1_HSTRING* previousStrings,
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNextWordCandidatesAsync(
                        UINT32 maxCandidates,
                        __FIIterable_1_HSTRING* previousStrings,
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputScope(
                        ABI::Windows::UI::Text::Core::CoreTextInputScope* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InputScope(
                        ABI::Windows::UI::Text::Core::CoreTextInputScope value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextPredictionGenerator2 = __uuidof(ITextPredictionGenerator2);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextPredictionGenerator2;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Data.Text.ITextPredictionGeneratorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextPredictionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextPredictionGeneratorFactory[] = L"Windows.Data.Text.ITextPredictionGeneratorFactory";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("7257b416-8ba2-4751-9d30-9d85435653a2")
                ITextPredictionGeneratorFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING languageTag,
                        ABI::Windows::Data::Text::ITextPredictionGenerator** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextPredictionGeneratorFactory = __uuidof(ITextPredictionGeneratorFactory);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextReverseConversionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextReverseConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextReverseConversionGenerator[] = L"Windows.Data.Text.ITextReverseConversionGenerator";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("51e7f514-9c51-4d86-ae1b-b498fbad8313")
                ITextReverseConversionGenerator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LanguageAvailableButNotInstalled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConvertBackAsync(
                        HSTRING input,
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextReverseConversionGenerator = __uuidof(ITextReverseConversionGenerator);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextReverseConversionGenerator2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextReverseConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextReverseConversionGenerator2[] = L"Windows.Data.Text.ITextReverseConversionGenerator2";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("1aafd2ec-85d6-46fd-828a-3a4830fa6e18")
                ITextReverseConversionGenerator2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPhonemesAsync(
                        HSTRING input,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextReverseConversionGenerator2 = __uuidof(ITextReverseConversionGenerator2);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Data.Text.ITextReverseConversionGeneratorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextReverseConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextReverseConversionGeneratorFactory[] = L"Windows.Data.Text.ITextReverseConversionGeneratorFactory";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("63bed326-1fda-41f6-89d5-23ddea3c729a")
                ITextReverseConversionGeneratorFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING languageTag,
                        ABI::Windows::Data::Text::ITextReverseConversionGenerator** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextReverseConversionGeneratorFactory = __uuidof(ITextReverseConversionGeneratorFactory);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IUnicodeCharactersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.UnicodeCharacters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IUnicodeCharactersStatics[] = L"Windows.Data.Text.IUnicodeCharactersStatics";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("97909e87-9291-4f91-b6c8-b6e359d7a7fb")
                IUnicodeCharactersStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCodepointFromSurrogatePair(
                        UINT32 highSurrogate,
                        UINT32 lowSurrogate,
                        UINT32* codepoint
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSurrogatePairFromCodepoint(
                        UINT32 codepoint,
                        WCHAR* highSurrogate,
                        WCHAR* lowSurrogate
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsHighSurrogate(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsLowSurrogate(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsSupplementary(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsNoncharacter(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsWhitespace(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsAlphabetic(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsCased(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsUppercase(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsLowercase(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsIdStart(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsIdContinue(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsGraphemeBase(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsGraphemeExtend(
                        UINT32 codepoint,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNumericType(
                        UINT32 codepoint,
                        ABI::Windows::Data::Text::UnicodeNumericType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetGeneralCategory(
                        UINT32 codepoint,
                        ABI::Windows::Data::Text::UnicodeGeneralCategory* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUnicodeCharactersStatics = __uuidof(IUnicodeCharactersStatics);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IWordSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.WordSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIWordSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIWordSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IWordSegment[] = L"Windows.Data.Text.IWordSegment";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("d2d4ba6d-987c-4cc0-b6bd-d49a11b38f9a")
                IWordSegment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceTextSegment(
                        ABI::Windows::Data::Text::TextSegment* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlternateForms(
                        __FIVectorView_1_Windows__CData__CText__CAlternateWordForm** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWordSegment = __uuidof(IWordSegment);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIWordSegment;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIWordSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IWordsSegmenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.WordsSegmenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIWordsSegmenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIWordsSegmenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IWordsSegmenter[] = L"Windows.Data.Text.IWordsSegmenter";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("86b4d4d1-b2fe-4e34-a81d-66640300454f")
                IWordsSegmenter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ResolvedLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTokenAt(
                        HSTRING text,
                        UINT32 startIndex,
                        ABI::Windows::Data::Text::IWordSegment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTokens(
                        HSTRING text,
                        __FIVectorView_1_Windows__CData__CText__CWordSegment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Tokenize(
                        HSTRING text,
                        UINT32 startIndex,
                        ABI::Windows::Data::Text::IWordSegmentsTokenizingHandler* handler
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWordsSegmenter = __uuidof(IWordsSegmenter);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIWordsSegmenter;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIWordsSegmenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IWordsSegmenterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.WordsSegmenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IWordsSegmenterFactory[] = L"Windows.Data.Text.IWordsSegmenterFactory";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Text {
                MIDL_INTERFACE("e6977274-fc35-455c-8bfb-6d7f4653ca97")
                IWordsSegmenterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithLanguage(
                        HSTRING language,
                        ABI::Windows::Data::Text::IWordsSegmenter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWordsSegmenterFactory = __uuidof(IWordsSegmenterFactory);
            } /* Text */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.AlternateWordForm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.IAlternateWordForm ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_AlternateWordForm_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_AlternateWordForm_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_AlternateWordForm[] = L"Windows.Data.Text.AlternateWordForm";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.SelectableWordSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ISelectableWordSegment ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_SelectableWordSegment_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_SelectableWordSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_SelectableWordSegment[] = L"Windows.Data.Text.SelectableWordSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.SelectableWordsSegmenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ISelectableWordsSegmenterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ISelectableWordsSegmenter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_SelectableWordsSegmenter_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_SelectableWordsSegmenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_SelectableWordsSegmenter[] = L"Windows.Data.Text.SelectableWordsSegmenter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.SemanticTextQuery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ISemanticTextQueryFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ISemanticTextQuery ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_SemanticTextQuery_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_SemanticTextQuery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_SemanticTextQuery[] = L"Windows.Data.Text.SemanticTextQuery";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.TextConversionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ITextConversionGeneratorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ITextConversionGenerator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_TextConversionGenerator_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_TextConversionGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_TextConversionGenerator[] = L"Windows.Data.Text.TextConversionGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.TextPhoneme
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ITextPhoneme ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Data_Text_TextPhoneme_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_TextPhoneme_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_TextPhoneme[] = L"Windows.Data.Text.TextPhoneme";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Data.Text.TextPredictionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ITextPredictionGeneratorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ITextPredictionGenerator ** Default Interface **
 *    Windows.Data.Text.ITextPredictionGenerator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_TextPredictionGenerator_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_TextPredictionGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_TextPredictionGenerator[] = L"Windows.Data.Text.TextPredictionGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.TextReverseConversionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ITextReverseConversionGeneratorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ITextReverseConversionGenerator ** Default Interface **
 *    Windows.Data.Text.ITextReverseConversionGenerator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_TextReverseConversionGenerator_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_TextReverseConversionGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_TextReverseConversionGenerator[] = L"Windows.Data.Text.TextReverseConversionGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.UnicodeCharacters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Data.Text.IUnicodeCharactersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_UnicodeCharacters_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_UnicodeCharacters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_UnicodeCharacters[] = L"Windows.Data.Text.UnicodeCharacters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.WordSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.IWordSegment ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_WordSegment_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_WordSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_WordSegment[] = L"Windows.Data.Text.WordSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.WordsSegmenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.IWordsSegmenterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.IWordsSegmenter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_WordsSegmenter_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_WordsSegmenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_WordsSegmenter[] = L"Windows.Data.Text.WordsSegmenter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler;

#endif // ____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler;

#endif // ____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIAlternateWordForm_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIAlternateWordForm_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CIAlternateWordForm __x_ABI_CWindows_CData_CText_CIAlternateWordForm;

#endif // ____x_ABI_CWindows_CData_CText_CIAlternateWordForm_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISelectableWordSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISelectableWordSegment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CISelectableWordSegment __x_ABI_CWindows_CData_CText_CISelectableWordSegment;

#endif // ____x_ABI_CWindows_CData_CText_CISelectableWordSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter;

#endif // ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory;

#endif // ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISemanticTextQuery_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISemanticTextQuery_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CISemanticTextQuery __x_ABI_CWindows_CData_CText_CISemanticTextQuery;

#endif // ____x_ABI_CWindows_CData_CText_CISemanticTextQuery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory;

#endif // ____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextConversionGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextConversionGenerator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextConversionGenerator __x_ABI_CWindows_CData_CText_CITextConversionGenerator;

#endif // ____x_ABI_CWindows_CData_CText_CITextConversionGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory;

#endif // ____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextPhoneme_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextPhoneme_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextPhoneme __x_ABI_CWindows_CData_CText_CITextPhoneme;

#endif // ____x_ABI_CWindows_CData_CText_CITextPhoneme_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextPredictionGenerator __x_ABI_CWindows_CData_CText_CITextPredictionGenerator;

#endif // ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2 __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2;

#endif // ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory;

#endif // ____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator;

#endif // ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2 __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2;

#endif // ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory;

#endif // ____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics;

#endif // ____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIWordSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIWordSegment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CIWordSegment __x_ABI_CWindows_CData_CText_CIWordSegment;

#endif // ____x_ABI_CWindows_CData_CText_CIWordSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIWordsSegmenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIWordsSegmenter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CIWordsSegmenter __x_ABI_CWindows_CData_CText_CIWordsSegmenter;

#endif // ____x_ABI_CWindows_CData_CText_CIWordsSegmenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory;

#endif // ____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

#if !defined(____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_HSTRING __FIAsyncOperation_1___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_HSTRING;

typedef struct __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CData__CText__CTextPhoneme __FIIterator_1_Windows__CData__CText__CTextPhoneme;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CData__CText__CTextPhoneme;

typedef struct __FIIterator_1_Windows__CData__CText__CTextPhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This,
        __x_ABI_CWindows_CData_CText_CITextPhoneme** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CData__CText__CTextPhoneme* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CText_CITextPhoneme** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CData__CText__CTextPhonemeVtbl;

interface __FIIterator_1_Windows__CData__CText__CTextPhoneme
{
    CONST_VTBL struct __FIIterator_1_Windows__CData__CText__CTextPhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CData__CText__CTextPhoneme_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CData__CText__CTextPhoneme __FIIterable_1_Windows__CData__CText__CTextPhoneme;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CData__CText__CTextPhoneme;

typedef struct __FIIterable_1_Windows__CData__CText__CTextPhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CData__CText__CTextPhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CData__CText__CTextPhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CData__CText__CTextPhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CData__CText__CTextPhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CData__CText__CTextPhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CData__CText__CTextPhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CData__CText__CTextPhoneme* This,
        __FIIterator_1_Windows__CData__CText__CTextPhoneme** result);

    END_INTERFACE
} __FIIterable_1_Windows__CData__CText__CTextPhonemeVtbl;

interface __FIIterable_1_Windows__CData__CText__CTextPhoneme
{
    CONST_VTBL struct __FIIterable_1_Windows__CData__CText__CTextPhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CData__CText__CTextPhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CData__CText__CTextPhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CData__CText__CTextPhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CData__CText__CTextPhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CData__CText__CTextPhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CData__CText__CTextPhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CData__CText__CTextPhoneme_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CData__CText__CTextPhoneme __FIVectorView_1_Windows__CData__CText__CTextPhoneme;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CData__CText__CTextPhoneme;

typedef struct __FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        UINT32 index,
        __x_ABI_CWindows_CData_CText_CITextPhoneme** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        __x_ABI_CWindows_CData_CText_CITextPhoneme* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CText_CITextPhoneme** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl;

interface __FIVectorView_1_Windows__CData__CText__CTextPhoneme
{
    CONST_VTBL struct __FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        __FIVectorView_1_Windows__CData__CText__CTextPhoneme** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CData__CText__CAlternateWordForm __FIIterator_1_Windows__CData__CText__CAlternateWordForm;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CData__CText__CAlternateWordForm;

typedef struct __FIIterator_1_Windows__CData__CText__CAlternateWordFormVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This,
        __x_ABI_CWindows_CData_CText_CIAlternateWordForm** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CData__CText__CAlternateWordForm* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CText_CIAlternateWordForm** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CData__CText__CAlternateWordFormVtbl;

interface __FIIterator_1_Windows__CData__CText__CAlternateWordForm
{
    CONST_VTBL struct __FIIterator_1_Windows__CData__CText__CAlternateWordFormVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CData__CText__CAlternateWordForm_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CData__CText__CAlternateWordForm __FIIterable_1_Windows__CData__CText__CAlternateWordForm;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CData__CText__CAlternateWordForm;

typedef struct __FIIterable_1_Windows__CData__CText__CAlternateWordFormVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CData__CText__CAlternateWordForm* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CData__CText__CAlternateWordForm* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CData__CText__CAlternateWordForm* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CData__CText__CAlternateWordForm* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CData__CText__CAlternateWordForm* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CData__CText__CAlternateWordForm* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CData__CText__CAlternateWordForm* This,
        __FIIterator_1_Windows__CData__CText__CAlternateWordForm** result);

    END_INTERFACE
} __FIIterable_1_Windows__CData__CText__CAlternateWordFormVtbl;

interface __FIIterable_1_Windows__CData__CText__CAlternateWordForm
{
    CONST_VTBL struct __FIIterable_1_Windows__CData__CText__CAlternateWordFormVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CData__CText__CAlternateWordForm_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CData__CText__CAlternateWordForm_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CData__CText__CAlternateWordForm_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CData__CText__CAlternateWordForm_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CData__CText__CAlternateWordForm_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CData__CText__CAlternateWordForm_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CData__CText__CAlternateWordForm_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CData__CText__CSelectableWordSegment __FIIterator_1_Windows__CData__CText__CSelectableWordSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CData__CText__CSelectableWordSegment;

typedef struct __FIIterator_1_Windows__CData__CText__CSelectableWordSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This,
        __x_ABI_CWindows_CData_CText_CISelectableWordSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CData__CText__CSelectableWordSegment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CText_CISelectableWordSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CData__CText__CSelectableWordSegmentVtbl;

interface __FIIterator_1_Windows__CData__CText__CSelectableWordSegment
{
    CONST_VTBL struct __FIIterator_1_Windows__CData__CText__CSelectableWordSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CData__CText__CSelectableWordSegment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CData__CText__CSelectableWordSegment __FIIterable_1_Windows__CData__CText__CSelectableWordSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CData__CText__CSelectableWordSegment;

typedef struct __FIIterable_1_Windows__CData__CText__CSelectableWordSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CData__CText__CSelectableWordSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CData__CText__CSelectableWordSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CData__CText__CSelectableWordSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CData__CText__CSelectableWordSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CData__CText__CSelectableWordSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CData__CText__CSelectableWordSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CData__CText__CSelectableWordSegment* This,
        __FIIterator_1_Windows__CData__CText__CSelectableWordSegment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CData__CText__CSelectableWordSegmentVtbl;

interface __FIIterable_1_Windows__CData__CText__CSelectableWordSegment
{
    CONST_VTBL struct __FIIterable_1_Windows__CData__CText__CSelectableWordSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CData__CText__CSelectableWordSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CData__CText__CSelectableWordSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CData__CText__CSelectableWordSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CData__CText__CSelectableWordSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CData__CText__CSelectableWordSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CData__CText__CSelectableWordSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CData__CText__CSelectableWordSegment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CData_CText_CTextSegment __x_ABI_CWindows_CData_CText_CTextSegment;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CData__CText__CTextSegment __FIIterator_1_Windows__CData__CText__CTextSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CData__CText__CTextSegment;

typedef struct __FIIterator_1_Windows__CData__CText__CTextSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CData__CText__CTextSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CData__CText__CTextSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CData__CText__CTextSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CData__CText__CTextSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CData__CText__CTextSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CData__CText__CTextSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CData__CText__CTextSegment* This,
        struct __x_ABI_CWindows_CData_CText_CTextSegment* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CData__CText__CTextSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CData__CText__CTextSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CData__CText__CTextSegment* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CData_CText_CTextSegment* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CData__CText__CTextSegmentVtbl;

interface __FIIterator_1_Windows__CData__CText__CTextSegment
{
    CONST_VTBL struct __FIIterator_1_Windows__CData__CText__CTextSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CData__CText__CTextSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CData__CText__CTextSegment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CData__CText__CTextSegment __FIIterable_1_Windows__CData__CText__CTextSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CData__CText__CTextSegment;

typedef struct __FIIterable_1_Windows__CData__CText__CTextSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CData__CText__CTextSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CData__CText__CTextSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CData__CText__CTextSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CData__CText__CTextSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CData__CText__CTextSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CData__CText__CTextSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CData__CText__CTextSegment* This,
        __FIIterator_1_Windows__CData__CText__CTextSegment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CData__CText__CTextSegmentVtbl;

interface __FIIterable_1_Windows__CData__CText__CTextSegment
{
    CONST_VTBL struct __FIIterable_1_Windows__CData__CText__CTextSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CData__CText__CTextSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CData__CText__CTextSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CData__CText__CTextSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CData__CText__CTextSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CData__CText__CTextSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CData__CText__CTextSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CData__CText__CTextSegment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CData__CText__CWordSegment __FIIterator_1_Windows__CData__CText__CWordSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CData__CText__CWordSegment;

typedef struct __FIIterator_1_Windows__CData__CText__CWordSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CData__CText__CWordSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CData__CText__CWordSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CData__CText__CWordSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CData__CText__CWordSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CData__CText__CWordSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CData__CText__CWordSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CData__CText__CWordSegment* This,
        __x_ABI_CWindows_CData_CText_CIWordSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CData__CText__CWordSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CData__CText__CWordSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CData__CText__CWordSegment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CText_CIWordSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CData__CText__CWordSegmentVtbl;

interface __FIIterator_1_Windows__CData__CText__CWordSegment
{
    CONST_VTBL struct __FIIterator_1_Windows__CData__CText__CWordSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CData__CText__CWordSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CData__CText__CWordSegment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CData__CText__CWordSegment __FIIterable_1_Windows__CData__CText__CWordSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CData__CText__CWordSegment;

typedef struct __FIIterable_1_Windows__CData__CText__CWordSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CData__CText__CWordSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CData__CText__CWordSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CData__CText__CWordSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CData__CText__CWordSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CData__CText__CWordSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CData__CText__CWordSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CData__CText__CWordSegment* This,
        __FIIterator_1_Windows__CData__CText__CWordSegment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CData__CText__CWordSegmentVtbl;

interface __FIIterable_1_Windows__CData__CText__CWordSegment
{
    CONST_VTBL struct __FIIterable_1_Windows__CData__CText__CWordSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CData__CText__CWordSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CData__CText__CWordSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CData__CText__CWordSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CData__CText__CWordSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CData__CText__CWordSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CData__CText__CWordSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CData__CText__CWordSegment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CData__CText__CAlternateWordForm __FIVectorView_1_Windows__CData__CText__CAlternateWordForm;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CData__CText__CAlternateWordForm;

typedef struct __FIVectorView_1_Windows__CData__CText__CAlternateWordFormVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This,
        UINT32 index,
        __x_ABI_CWindows_CData_CText_CIAlternateWordForm** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This,
        __x_ABI_CWindows_CData_CText_CIAlternateWordForm* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CData__CText__CAlternateWordForm* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CText_CIAlternateWordForm** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CData__CText__CAlternateWordFormVtbl;

interface __FIVectorView_1_Windows__CData__CText__CAlternateWordForm
{
    CONST_VTBL struct __FIVectorView_1_Windows__CData__CText__CAlternateWordFormVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CData__CText__CAlternateWordForm_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CData__CText__CAlternateWordForm_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CData__CText__CSelectableWordSegment;

typedef struct __FIVectorView_1_Windows__CData__CText__CSelectableWordSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This,
        UINT32 index,
        __x_ABI_CWindows_CData_CText_CISelectableWordSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This,
        __x_ABI_CWindows_CData_CText_CISelectableWordSegment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CData__CText__CSelectableWordSegment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CText_CISelectableWordSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CData__CText__CSelectableWordSegmentVtbl;

interface __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CData__CText__CSelectableWordSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CData__CText__CSelectableWordSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CData__CText__CTextSegment __FIVectorView_1_Windows__CData__CText__CTextSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CData__CText__CTextSegment;

typedef struct __FIVectorView_1_Windows__CData__CText__CTextSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This,
        UINT32 index,
        struct __x_ABI_CWindows_CData_CText_CTextSegment* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This,
        struct __x_ABI_CWindows_CData_CText_CTextSegment value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CData__CText__CTextSegment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CData_CText_CTextSegment* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CData__CText__CTextSegmentVtbl;

interface __FIVectorView_1_Windows__CData__CText__CTextSegment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CData__CText__CTextSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CData__CText__CTextSegment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CData__CText__CTextSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CData__CText__CWordSegment __FIVectorView_1_Windows__CData__CText__CWordSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CData__CText__CWordSegment;

typedef struct __FIVectorView_1_Windows__CData__CText__CWordSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This,
        UINT32 index,
        __x_ABI_CWindows_CData_CText_CIWordSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This,
        __x_ABI_CWindows_CData_CText_CIWordSegment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CData__CText__CWordSegment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CText_CIWordSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CData__CText__CWordSegmentVtbl;

interface __FIVectorView_1_Windows__CData__CText__CWordSegment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CData__CText__CWordSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CData__CText__CWordSegment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CData__CText__CWordSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope;

typedef enum __x_ABI_CWindows_CData_CText_CAlternateNormalizationFormat __x_ABI_CWindows_CData_CText_CAlternateNormalizationFormat;

typedef enum __x_ABI_CWindows_CData_CText_CTextPredictionOptions __x_ABI_CWindows_CData_CText_CTextPredictionOptions;

typedef enum __x_ABI_CWindows_CData_CText_CUnicodeGeneralCategory __x_ABI_CWindows_CData_CText_CUnicodeGeneralCategory;

typedef enum __x_ABI_CWindows_CData_CText_CUnicodeNumericType __x_ABI_CWindows_CData_CText_CUnicodeNumericType;

/*
 *
 * Struct Windows.Data.Text.AlternateNormalizationFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CData_CText_CAlternateNormalizationFormat
{
    AlternateNormalizationFormat_NotNormalized = 0,
    AlternateNormalizationFormat_Number = 1,
    AlternateNormalizationFormat_Currency = 3,
    AlternateNormalizationFormat_Date = 4,
    AlternateNormalizationFormat_Time = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Data.Text.TextPredictionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CData_CText_CTextPredictionOptions
{
    TextPredictionOptions_None = 0,
    TextPredictionOptions_Predictions = 0x1,
    TextPredictionOptions_Corrections = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Data.Text.UnicodeGeneralCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CData_CText_CUnicodeGeneralCategory
{
    UnicodeGeneralCategory_UppercaseLetter = 0,
    UnicodeGeneralCategory_LowercaseLetter = 1,
    UnicodeGeneralCategory_TitlecaseLetter = 2,
    UnicodeGeneralCategory_ModifierLetter = 3,
    UnicodeGeneralCategory_OtherLetter = 4,
    UnicodeGeneralCategory_NonspacingMark = 5,
    UnicodeGeneralCategory_SpacingCombiningMark = 6,
    UnicodeGeneralCategory_EnclosingMark = 7,
    UnicodeGeneralCategory_DecimalDigitNumber = 8,
    UnicodeGeneralCategory_LetterNumber = 9,
    UnicodeGeneralCategory_OtherNumber = 10,
    UnicodeGeneralCategory_SpaceSeparator = 11,
    UnicodeGeneralCategory_LineSeparator = 12,
    UnicodeGeneralCategory_ParagraphSeparator = 13,
    UnicodeGeneralCategory_Control = 14,
    UnicodeGeneralCategory_Format = 15,
    UnicodeGeneralCategory_Surrogate = 16,
    UnicodeGeneralCategory_PrivateUse = 17,
    UnicodeGeneralCategory_ConnectorPunctuation = 18,
    UnicodeGeneralCategory_DashPunctuation = 19,
    UnicodeGeneralCategory_OpenPunctuation = 20,
    UnicodeGeneralCategory_ClosePunctuation = 21,
    UnicodeGeneralCategory_InitialQuotePunctuation = 22,
    UnicodeGeneralCategory_FinalQuotePunctuation = 23,
    UnicodeGeneralCategory_OtherPunctuation = 24,
    UnicodeGeneralCategory_MathSymbol = 25,
    UnicodeGeneralCategory_CurrencySymbol = 26,
    UnicodeGeneralCategory_ModifierSymbol = 27,
    UnicodeGeneralCategory_OtherSymbol = 28,
    UnicodeGeneralCategory_NotAssigned = 29,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Data.Text.UnicodeNumericType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CData_CText_CUnicodeNumericType
{
    UnicodeNumericType_None = 0,
    UnicodeNumericType_Decimal = 1,
    UnicodeNumericType_Digit = 2,
    UnicodeNumericType_Numeric = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Data.Text.TextSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CData_CText_CTextSegment
{
    UINT32 StartPosition;
    UINT32 Length;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Data.Text.SelectableWordSegmentsTokenizingHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler* This,
        __FIIterable_1_Windows__CData__CText__CSelectableWordSegment* precedingWords,
        __FIIterable_1_Windows__CData__CText__CSelectableWordSegment* words);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandlerVtbl;

interface __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_Invoke(This, precedingWords, words) \
    ((This)->lpVtbl->Invoke(This, precedingWords, words))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Data.Text.WordSegmentsTokenizingHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler* This,
        __FIIterable_1_Windows__CData__CText__CWordSegment* precedingWords,
        __FIIterable_1_Windows__CData__CText__CWordSegment* words);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandlerVtbl;

interface __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_Invoke(This, precedingWords, words) \
    ((This)->lpVtbl->Invoke(This, precedingWords, words))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IAlternateWordForm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.AlternateWordForm
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIAlternateWordForm_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIAlternateWordForm_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IAlternateWordForm[] = L"Windows.Data.Text.IAlternateWordForm";
typedef struct __x_ABI_CWindows_CData_CText_CIAlternateWordFormVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceTextSegment)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This,
        struct __x_ABI_CWindows_CData_CText_CTextSegment* value);
    HRESULT (STDMETHODCALLTYPE* get_AlternateText)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NormalizationFormat)(__x_ABI_CWindows_CData_CText_CIAlternateWordForm* This,
        enum __x_ABI_CWindows_CData_CText_CAlternateNormalizationFormat* value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CIAlternateWordFormVtbl;

interface __x_ABI_CWindows_CData_CText_CIAlternateWordForm
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CIAlternateWordFormVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_get_SourceTextSegment(This, value) \
    ((This)->lpVtbl->get_SourceTextSegment(This, value))

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_get_AlternateText(This, value) \
    ((This)->lpVtbl->get_AlternateText(This, value))

#define __x_ABI_CWindows_CData_CText_CIAlternateWordForm_get_NormalizationFormat(This, value) \
    ((This)->lpVtbl->get_NormalizationFormat(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIAlternateWordForm;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIAlternateWordForm_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISelectableWordSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SelectableWordSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISelectableWordSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISelectableWordSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISelectableWordSegment[] = L"Windows.Data.Text.ISelectableWordSegment";
typedef struct __x_ABI_CWindows_CData_CText_CISelectableWordSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CISelectableWordSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CISelectableWordSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CISelectableWordSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CISelectableWordSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CISelectableWordSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CISelectableWordSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CData_CText_CISelectableWordSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceTextSegment)(__x_ABI_CWindows_CData_CText_CISelectableWordSegment* This,
        struct __x_ABI_CWindows_CData_CText_CTextSegment* value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CISelectableWordSegmentVtbl;

interface __x_ABI_CWindows_CData_CText_CISelectableWordSegment
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CISelectableWordSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CData_CText_CISelectableWordSegment_get_SourceTextSegment(This, value) \
    ((This)->lpVtbl->get_SourceTextSegment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISelectableWordSegment;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISelectableWordSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISelectableWordsSegmenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SelectableWordsSegmenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISelectableWordsSegmenter[] = L"Windows.Data.Text.ISelectableWordsSegmenter";
typedef struct __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetTokenAt)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This,
        HSTRING text,
        UINT32 startIndex,
        __x_ABI_CWindows_CData_CText_CISelectableWordSegment** result);
    HRESULT (STDMETHODCALLTYPE* GetTokens)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This,
        HSTRING text,
        __FIVectorView_1_Windows__CData__CText__CSelectableWordSegment** result);
    HRESULT (STDMETHODCALLTYPE* Tokenize)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter* This,
        HSTRING text,
        UINT32 startIndex,
        __x_ABI_CWindows_CData_CText_CISelectableWordSegmentsTokenizingHandler* handler);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterVtbl;

interface __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_GetTokenAt(This, text, startIndex, result) \
    ((This)->lpVtbl->GetTokenAt(This, text, startIndex, result))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_GetTokens(This, text, result) \
    ((This)->lpVtbl->GetTokens(This, text, result))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_Tokenize(This, text, startIndex, handler) \
    ((This)->lpVtbl->Tokenize(This, text, startIndex, handler))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISelectableWordsSegmenterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SelectableWordsSegmenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISelectableWordsSegmenterFactory[] = L"Windows.Data.Text.ISelectableWordsSegmenterFactory";
typedef struct __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithLanguage)(__x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory* This,
        HSTRING language,
        __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenter** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactoryVtbl;

interface __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_CreateWithLanguage(This, language, result) \
    ((This)->lpVtbl->CreateWithLanguage(This, language, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISelectableWordsSegmenterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISemanticTextQuery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SemanticTextQuery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISemanticTextQuery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISemanticTextQuery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISemanticTextQuery[] = L"Windows.Data.Text.ISemanticTextQuery";
typedef struct __x_ABI_CWindows_CData_CText_CISemanticTextQueryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CISemanticTextQuery* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CISemanticTextQuery* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CISemanticTextQuery* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CISemanticTextQuery* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CISemanticTextQuery* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CISemanticTextQuery* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Find)(__x_ABI_CWindows_CData_CText_CISemanticTextQuery* This,
        HSTRING content,
        __FIVectorView_1_Windows__CData__CText__CTextSegment** result);
    HRESULT (STDMETHODCALLTYPE* FindInProperty)(__x_ABI_CWindows_CData_CText_CISemanticTextQuery* This,
        HSTRING propertyContent,
        HSTRING propertyName,
        __FIVectorView_1_Windows__CData__CText__CTextSegment** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CISemanticTextQueryVtbl;

interface __x_ABI_CWindows_CData_CText_CISemanticTextQuery
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CISemanticTextQueryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery_Find(This, content, result) \
    ((This)->lpVtbl->Find(This, content, result))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQuery_FindInProperty(This, propertyContent, propertyName, result) \
    ((This)->lpVtbl->FindInProperty(This, propertyContent, propertyName, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISemanticTextQuery;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISemanticTextQuery_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ISemanticTextQueryFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.SemanticTextQuery
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ISemanticTextQueryFactory[] = L"Windows.Data.Text.ISemanticTextQueryFactory";
typedef struct __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory* This,
        HSTRING aqsFilter,
        __x_ABI_CWindows_CData_CText_CISemanticTextQuery** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithLanguage)(__x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory* This,
        HSTRING aqsFilter,
        HSTRING filterLanguage,
        __x_ABI_CWindows_CData_CText_CISemanticTextQuery** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactoryVtbl;

interface __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_Create(This, aqsFilter, result) \
    ((This)->lpVtbl->Create(This, aqsFilter, result))

#define __x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_CreateWithLanguage(This, aqsFilter, filterLanguage, result) \
    ((This)->lpVtbl->CreateWithLanguage(This, aqsFilter, filterLanguage, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CISemanticTextQueryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextConversionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextConversionGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextConversionGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextConversionGenerator[] = L"Windows.Data.Text.ITextConversionGenerator";
typedef struct __x_ABI_CWindows_CData_CText_CITextConversionGeneratorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LanguageAvailableButNotInstalled)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetCandidatesAsync)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This,
        HSTRING input,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetCandidatesWithMaxCountAsync)(__x_ABI_CWindows_CData_CText_CITextConversionGenerator* This,
        HSTRING input,
        UINT32 maxCandidates,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextConversionGeneratorVtbl;

interface __x_ABI_CWindows_CData_CText_CITextConversionGenerator
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextConversionGeneratorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_get_LanguageAvailableButNotInstalled(This, value) \
    ((This)->lpVtbl->get_LanguageAvailableButNotInstalled(This, value))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_GetCandidatesAsync(This, input, result) \
    ((This)->lpVtbl->GetCandidatesAsync(This, input, result))

#define __x_ABI_CWindows_CData_CText_CITextConversionGenerator_GetCandidatesWithMaxCountAsync(This, input, maxCandidates, result) \
    ((This)->lpVtbl->GetCandidatesWithMaxCountAsync(This, input, maxCandidates, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextConversionGenerator;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextConversionGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextConversionGeneratorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextConversionGeneratorFactory[] = L"Windows.Data.Text.ITextConversionGeneratorFactory";
typedef struct __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory* This,
        HSTRING languageTag,
        __x_ABI_CWindows_CData_CText_CITextConversionGenerator** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactoryVtbl;

interface __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_Create(This, languageTag, result) \
    ((This)->lpVtbl->Create(This, languageTag, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextConversionGeneratorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextPhoneme
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextPhoneme
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CData_CText_CITextPhoneme_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextPhoneme_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextPhoneme[] = L"Windows.Data.Text.ITextPhoneme";
typedef struct __x_ABI_CWindows_CData_CText_CITextPhonemeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextPhoneme* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextPhoneme* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextPhoneme* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextPhoneme* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextPhoneme* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextPhoneme* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayText)(__x_ABI_CWindows_CData_CText_CITextPhoneme* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ReadingText)(__x_ABI_CWindows_CData_CText_CITextPhoneme* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextPhonemeVtbl;

interface __x_ABI_CWindows_CData_CText_CITextPhoneme
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextPhonemeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextPhoneme_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextPhoneme_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextPhoneme_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextPhoneme_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextPhoneme_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextPhoneme_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextPhoneme_get_DisplayText(This, value) \
    ((This)->lpVtbl->get_DisplayText(This, value))

#define __x_ABI_CWindows_CData_CText_CITextPhoneme_get_ReadingText(This, value) \
    ((This)->lpVtbl->get_ReadingText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextPhoneme;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextPhoneme_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Data.Text.ITextPredictionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextPredictionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextPredictionGenerator[] = L"Windows.Data.Text.ITextPredictionGenerator";
typedef struct __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LanguageAvailableButNotInstalled)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetCandidatesAsync)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This,
        HSTRING input,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetCandidatesWithMaxCountAsync)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator* This,
        HSTRING input,
        UINT32 maxCandidates,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorVtbl;

interface __x_ABI_CWindows_CData_CText_CITextPredictionGenerator
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_get_LanguageAvailableButNotInstalled(This, value) \
    ((This)->lpVtbl->get_LanguageAvailableButNotInstalled(This, value))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_GetCandidatesAsync(This, input, result) \
    ((This)->lpVtbl->GetCandidatesAsync(This, input, result))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator_GetCandidatesWithMaxCountAsync(This, input, maxCandidates, result) \
    ((This)->lpVtbl->GetCandidatesWithMaxCountAsync(This, input, maxCandidates, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextPredictionGenerator;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextPredictionGenerator2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextPredictionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextPredictionGenerator2[] = L"Windows.Data.Text.ITextPredictionGenerator2";
typedef struct __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCandidatesWithParametersAsync)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This,
        HSTRING input,
        UINT32 maxCandidates,
        enum __x_ABI_CWindows_CData_CText_CTextPredictionOptions predictionOptions,
        __FIIterable_1_HSTRING* previousStrings,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetNextWordCandidatesAsync)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This,
        UINT32 maxCandidates,
        __FIIterable_1_HSTRING* previousStrings,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_InputScope)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope* value);
    HRESULT (STDMETHODCALLTYPE* put_InputScope)(__x_ABI_CWindows_CData_CText_CITextPredictionGenerator2* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2Vtbl;

interface __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_GetCandidatesWithParametersAsync(This, input, maxCandidates, predictionOptions, previousStrings, result) \
    ((This)->lpVtbl->GetCandidatesWithParametersAsync(This, input, maxCandidates, predictionOptions, previousStrings, result))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_GetNextWordCandidatesAsync(This, maxCandidates, previousStrings, result) \
    ((This)->lpVtbl->GetNextWordCandidatesAsync(This, maxCandidates, previousStrings, result))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_get_InputScope(This, value) \
    ((This)->lpVtbl->get_InputScope(This, value))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_put_InputScope(This, value) \
    ((This)->lpVtbl->put_InputScope(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextPredictionGenerator2;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGenerator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Data.Text.ITextPredictionGeneratorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextPredictionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextPredictionGeneratorFactory[] = L"Windows.Data.Text.ITextPredictionGeneratorFactory";
typedef struct __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory* This,
        HSTRING languageTag,
        __x_ABI_CWindows_CData_CText_CITextPredictionGenerator** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactoryVtbl;

interface __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_Create(This, languageTag, result) \
    ((This)->lpVtbl->Create(This, languageTag, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextPredictionGeneratorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextReverseConversionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextReverseConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextReverseConversionGenerator[] = L"Windows.Data.Text.ITextReverseConversionGenerator";
typedef struct __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LanguageAvailableButNotInstalled)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ConvertBackAsync)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator* This,
        HSTRING input,
        __FIAsyncOperation_1_HSTRING** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorVtbl;

interface __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_get_LanguageAvailableButNotInstalled(This, value) \
    ((This)->lpVtbl->get_LanguageAvailableButNotInstalled(This, value))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_ConvertBackAsync(This, input, result) \
    ((This)->lpVtbl->ConvertBackAsync(This, input, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.ITextReverseConversionGenerator2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextReverseConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextReverseConversionGenerator2[] = L"Windows.Data.Text.ITextReverseConversionGenerator2";
typedef struct __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPhonemesAsync)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2* This,
        HSTRING input,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CData__CText__CTextPhoneme** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2Vtbl;

interface __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_GetPhonemesAsync(This, input, result) \
    ((This)->lpVtbl->GetPhonemesAsync(This, input, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Data.Text.ITextReverseConversionGeneratorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.TextReverseConversionGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_ITextReverseConversionGeneratorFactory[] = L"Windows.Data.Text.ITextReverseConversionGeneratorFactory";
typedef struct __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory* This,
        HSTRING languageTag,
        __x_ABI_CWindows_CData_CText_CITextReverseConversionGenerator** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactoryVtbl;

interface __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_Create(This, languageTag, result) \
    ((This)->lpVtbl->Create(This, languageTag, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CITextReverseConversionGeneratorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IUnicodeCharactersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.UnicodeCharacters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IUnicodeCharactersStatics[] = L"Windows.Data.Text.IUnicodeCharactersStatics";
typedef struct __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCodepointFromSurrogatePair)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 highSurrogate,
        UINT32 lowSurrogate,
        UINT32* codepoint);
    HRESULT (STDMETHODCALLTYPE* GetSurrogatePairFromCodepoint)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        WCHAR* highSurrogate,
        WCHAR* lowSurrogate);
    HRESULT (STDMETHODCALLTYPE* IsHighSurrogate)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsLowSurrogate)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsSupplementary)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsNoncharacter)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsWhitespace)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsAlphabetic)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsCased)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsUppercase)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsLowercase)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsIdStart)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsIdContinue)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsGraphemeBase)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsGraphemeExtend)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetNumericType)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        enum __x_ABI_CWindows_CData_CText_CUnicodeNumericType* value);
    HRESULT (STDMETHODCALLTYPE* GetGeneralCategory)(__x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics* This,
        UINT32 codepoint,
        enum __x_ABI_CWindows_CData_CText_CUnicodeGeneralCategory* value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStaticsVtbl;

interface __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_GetCodepointFromSurrogatePair(This, highSurrogate, lowSurrogate, codepoint) \
    ((This)->lpVtbl->GetCodepointFromSurrogatePair(This, highSurrogate, lowSurrogate, codepoint))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_GetSurrogatePairFromCodepoint(This, codepoint, highSurrogate, lowSurrogate) \
    ((This)->lpVtbl->GetSurrogatePairFromCodepoint(This, codepoint, highSurrogate, lowSurrogate))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsHighSurrogate(This, codepoint, value) \
    ((This)->lpVtbl->IsHighSurrogate(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsLowSurrogate(This, codepoint, value) \
    ((This)->lpVtbl->IsLowSurrogate(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsSupplementary(This, codepoint, value) \
    ((This)->lpVtbl->IsSupplementary(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsNoncharacter(This, codepoint, value) \
    ((This)->lpVtbl->IsNoncharacter(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsWhitespace(This, codepoint, value) \
    ((This)->lpVtbl->IsWhitespace(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsAlphabetic(This, codepoint, value) \
    ((This)->lpVtbl->IsAlphabetic(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsCased(This, codepoint, value) \
    ((This)->lpVtbl->IsCased(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsUppercase(This, codepoint, value) \
    ((This)->lpVtbl->IsUppercase(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsLowercase(This, codepoint, value) \
    ((This)->lpVtbl->IsLowercase(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsIdStart(This, codepoint, value) \
    ((This)->lpVtbl->IsIdStart(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsIdContinue(This, codepoint, value) \
    ((This)->lpVtbl->IsIdContinue(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsGraphemeBase(This, codepoint, value) \
    ((This)->lpVtbl->IsGraphemeBase(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_IsGraphemeExtend(This, codepoint, value) \
    ((This)->lpVtbl->IsGraphemeExtend(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_GetNumericType(This, codepoint, value) \
    ((This)->lpVtbl->GetNumericType(This, codepoint, value))

#define __x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_GetGeneralCategory(This, codepoint, value) \
    ((This)->lpVtbl->GetGeneralCategory(This, codepoint, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIUnicodeCharactersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IWordSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.WordSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIWordSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIWordSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IWordSegment[] = L"Windows.Data.Text.IWordSegment";
typedef struct __x_ABI_CWindows_CData_CText_CIWordSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CIWordSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CIWordSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CIWordSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CIWordSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CIWordSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CIWordSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CData_CText_CIWordSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceTextSegment)(__x_ABI_CWindows_CData_CText_CIWordSegment* This,
        struct __x_ABI_CWindows_CData_CText_CTextSegment* value);
    HRESULT (STDMETHODCALLTYPE* get_AlternateForms)(__x_ABI_CWindows_CData_CText_CIWordSegment* This,
        __FIVectorView_1_Windows__CData__CText__CAlternateWordForm** value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CIWordSegmentVtbl;

interface __x_ABI_CWindows_CData_CText_CIWordSegment
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CIWordSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CIWordSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CIWordSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CIWordSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CIWordSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CIWordSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CIWordSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CIWordSegment_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CData_CText_CIWordSegment_get_SourceTextSegment(This, value) \
    ((This)->lpVtbl->get_SourceTextSegment(This, value))

#define __x_ABI_CWindows_CData_CText_CIWordSegment_get_AlternateForms(This, value) \
    ((This)->lpVtbl->get_AlternateForms(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIWordSegment;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIWordSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IWordsSegmenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.WordsSegmenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIWordsSegmenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIWordsSegmenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IWordsSegmenter[] = L"Windows.Data.Text.IWordsSegmenter";
typedef struct __x_ABI_CWindows_CData_CText_CIWordsSegmenterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResolvedLanguage)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetTokenAt)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This,
        HSTRING text,
        UINT32 startIndex,
        __x_ABI_CWindows_CData_CText_CIWordSegment** result);
    HRESULT (STDMETHODCALLTYPE* GetTokens)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This,
        HSTRING text,
        __FIVectorView_1_Windows__CData__CText__CWordSegment** result);
    HRESULT (STDMETHODCALLTYPE* Tokenize)(__x_ABI_CWindows_CData_CText_CIWordsSegmenter* This,
        HSTRING text,
        UINT32 startIndex,
        __x_ABI_CWindows_CData_CText_CIWordSegmentsTokenizingHandler* handler);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CIWordsSegmenterVtbl;

interface __x_ABI_CWindows_CData_CText_CIWordsSegmenter
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CIWordsSegmenterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_get_ResolvedLanguage(This, value) \
    ((This)->lpVtbl->get_ResolvedLanguage(This, value))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_GetTokenAt(This, text, startIndex, result) \
    ((This)->lpVtbl->GetTokenAt(This, text, startIndex, result))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_GetTokens(This, text, result) \
    ((This)->lpVtbl->GetTokens(This, text, result))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenter_Tokenize(This, text, startIndex, handler) \
    ((This)->lpVtbl->Tokenize(This, text, startIndex, handler))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIWordsSegmenter;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIWordsSegmenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Text.IWordsSegmenterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Text.WordsSegmenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Text_IWordsSegmenterFactory[] = L"Windows.Data.Text.IWordsSegmenterFactory";
typedef struct __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithLanguage)(__x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory* This,
        HSTRING language,
        __x_ABI_CWindows_CData_CText_CIWordsSegmenter** result);

    END_INTERFACE
} __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactoryVtbl;

interface __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_CreateWithLanguage(This, language, result) \
    ((This)->lpVtbl->CreateWithLanguage(This, language, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory;
#endif /* !defined(____x_ABI_CWindows_CData_CText_CIWordsSegmenterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.AlternateWordForm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.IAlternateWordForm ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_AlternateWordForm_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_AlternateWordForm_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_AlternateWordForm[] = L"Windows.Data.Text.AlternateWordForm";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.SelectableWordSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ISelectableWordSegment ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_SelectableWordSegment_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_SelectableWordSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_SelectableWordSegment[] = L"Windows.Data.Text.SelectableWordSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.SelectableWordsSegmenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ISelectableWordsSegmenterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ISelectableWordsSegmenter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_SelectableWordsSegmenter_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_SelectableWordsSegmenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_SelectableWordsSegmenter[] = L"Windows.Data.Text.SelectableWordsSegmenter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.SemanticTextQuery
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ISemanticTextQueryFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ISemanticTextQuery ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_SemanticTextQuery_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_SemanticTextQuery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_SemanticTextQuery[] = L"Windows.Data.Text.SemanticTextQuery";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.TextConversionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ITextConversionGeneratorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ITextConversionGenerator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_TextConversionGenerator_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_TextConversionGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_TextConversionGenerator[] = L"Windows.Data.Text.TextConversionGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.TextPhoneme
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ITextPhoneme ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Data_Text_TextPhoneme_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_TextPhoneme_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_TextPhoneme[] = L"Windows.Data.Text.TextPhoneme";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Data.Text.TextPredictionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ITextPredictionGeneratorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ITextPredictionGenerator ** Default Interface **
 *    Windows.Data.Text.ITextPredictionGenerator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_TextPredictionGenerator_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_TextPredictionGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_TextPredictionGenerator[] = L"Windows.Data.Text.TextPredictionGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.TextReverseConversionGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.ITextReverseConversionGeneratorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.ITextReverseConversionGenerator ** Default Interface **
 *    Windows.Data.Text.ITextReverseConversionGenerator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_TextReverseConversionGenerator_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_TextReverseConversionGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_TextReverseConversionGenerator[] = L"Windows.Data.Text.TextReverseConversionGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.UnicodeCharacters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Data.Text.IUnicodeCharactersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_UnicodeCharacters_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_UnicodeCharacters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_UnicodeCharacters[] = L"Windows.Data.Text.UnicodeCharacters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.WordSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.IWordSegment ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_WordSegment_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_WordSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_WordSegment[] = L"Windows.Data.Text.WordSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Text.WordsSegmenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Data.Text.IWordsSegmenterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Text.IWordsSegmenter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Text_WordsSegmenter_DEFINED
#define RUNTIMECLASS_Windows_Data_Text_WordsSegmenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Text_WordsSegmenter[] = L"Windows.Data.Text.WordsSegmenter";
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
#endif // __windows2Edata2Etext_p_h__

#endif // __windows2Edata2Etext_h__
