
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
#ifndef __windows2Emedia2Eocr_h__
#define __windows2Emedia2Eocr_h__
#ifndef __windows2Emedia2Eocr_p_h__
#define __windows2Emedia2Eocr_p_h__


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

#if !defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)
#define WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Globalization.h"
#include "Windows.Graphics.Imaging.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                interface IOcrEngine;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine ABI::Windows::Media::Ocr::IOcrEngine

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                interface IOcrEngineStatics;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics ABI::Windows::Media::Ocr::IOcrEngineStatics

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrLine_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrLine_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                interface IOcrLine;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine ABI::Windows::Media::Ocr::IOcrLine

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrLine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                interface IOcrResult;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult ABI::Windows::Media::Ocr::IOcrResult

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrWord_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrWord_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                interface IOcrWord;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord ABI::Windows::Media::Ocr::IOcrWord

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrWord_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                class OcrResult;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c7d7118e-ae36-59c0-ac76-7badee711c8b"))
IAsyncOperation<ABI::Windows::Media::Ocr::OcrResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Ocr::OcrResult*, ABI::Windows::Media::Ocr::IOcrResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Ocr.OcrResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Ocr::OcrResult*> __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("989c1371-444a-5e7e-b197-9eaaf9d2829a"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Ocr::OcrResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Ocr::OcrResult*, ABI::Windows::Media::Ocr::IOcrResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Ocr.OcrResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Ocr::OcrResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Globalization {
            class Language;
        } /* Globalization */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguage;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguage ABI::Windows::Globalization::ILanguage

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGlobalization__CLanguage_USE
#define DEF___FIIterator_1_Windows__CGlobalization__CLanguage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("30e99ae6-f414-5243-8db2-aab38ea3f1f1"))
IIterator<ABI::Windows::Globalization::Language*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Language*, ABI::Windows::Globalization::ILanguage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Globalization.Language>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Globalization::Language*> __FIIterator_1_Windows__CGlobalization__CLanguage_t;
#define __FIIterator_1_Windows__CGlobalization__CLanguage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGlobalization__CLanguage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGlobalization__CLanguage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGlobalization__CLanguage_USE
#define DEF___FIIterable_1_Windows__CGlobalization__CLanguage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("48409a10-61b6-5db1-a69d-8abc46ac608a"))
IIterable<ABI::Windows::Globalization::Language*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Language*, ABI::Windows::Globalization::ILanguage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Globalization.Language>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Globalization::Language*> __FIIterable_1_Windows__CGlobalization__CLanguage_t;
#define __FIIterable_1_Windows__CGlobalization__CLanguage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGlobalization__CLanguage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGlobalization__CLanguage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                class OcrLine;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__COcr__COcrLine_USE
#define DEF___FIIterator_1_Windows__CMedia__COcr__COcrLine_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("52ca0f8a-5788-5695-b905-46b8d8171d88"))
IIterator<ABI::Windows::Media::Ocr::OcrLine*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Ocr::OcrLine*, ABI::Windows::Media::Ocr::IOcrLine*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Ocr.OcrLine>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Ocr::OcrLine*> __FIIterator_1_Windows__CMedia__COcr__COcrLine_t;
#define __FIIterator_1_Windows__CMedia__COcr__COcrLine ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__COcr__COcrLine_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__COcr__COcrLine_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__COcr__COcrLine_USE
#define DEF___FIIterable_1_Windows__CMedia__COcr__COcrLine_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6afa94a2-60d7-5dbe-942d-81aa3929c85e"))
IIterable<ABI::Windows::Media::Ocr::OcrLine*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Ocr::OcrLine*, ABI::Windows::Media::Ocr::IOcrLine*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Ocr.OcrLine>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Ocr::OcrLine*> __FIIterable_1_Windows__CMedia__COcr__COcrLine_t;
#define __FIIterable_1_Windows__CMedia__COcr__COcrLine ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__COcr__COcrLine_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__COcr__COcrLine_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                class OcrWord;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__COcr__COcrWord_USE
#define DEF___FIIterator_1_Windows__CMedia__COcr__COcrWord_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0ed4317a-9964-51c6-acbe-02512a069082"))
IIterator<ABI::Windows::Media::Ocr::OcrWord*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Ocr::OcrWord*, ABI::Windows::Media::Ocr::IOcrWord*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Ocr.OcrWord>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Ocr::OcrWord*> __FIIterator_1_Windows__CMedia__COcr__COcrWord_t;
#define __FIIterator_1_Windows__CMedia__COcr__COcrWord ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__COcr__COcrWord_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__COcr__COcrWord_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__COcr__COcrWord_USE
#define DEF___FIIterable_1_Windows__CMedia__COcr__COcrWord_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a0ce663a-46d0-55e5-928e-251eb67a1e99"))
IIterable<ABI::Windows::Media::Ocr::OcrWord*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Ocr::OcrWord*, ABI::Windows::Media::Ocr::IOcrWord*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Ocr.OcrWord>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Ocr::OcrWord*> __FIIterable_1_Windows__CMedia__COcr__COcrWord_t;
#define __FIIterable_1_Windows__CMedia__COcr__COcrWord ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__COcr__COcrWord_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__COcr__COcrWord_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGlobalization__CLanguage_USE
#define DEF___FIVectorView_1_Windows__CGlobalization__CLanguage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("144b0f3d-2d59-5dd2-b012-908ec3e06435"))
IVectorView<ABI::Windows::Globalization::Language*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Language*, ABI::Windows::Globalization::ILanguage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Globalization.Language>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Globalization::Language*> __FIVectorView_1_Windows__CGlobalization__CLanguage_t;
#define __FIVectorView_1_Windows__CGlobalization__CLanguage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGlobalization__CLanguage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGlobalization__CLanguage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__COcr__COcrLine_USE
#define DEF___FIVectorView_1_Windows__CMedia__COcr__COcrLine_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60c76eac-8875-5ddb-a19b-65a3936279ea"))
IVectorView<ABI::Windows::Media::Ocr::OcrLine*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Ocr::OcrLine*, ABI::Windows::Media::Ocr::IOcrLine*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Ocr.OcrLine>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Ocr::OcrLine*> __FIVectorView_1_Windows__CMedia__COcr__COcrLine_t;
#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__COcr__COcrLine_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__COcr__COcrLine_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__COcr__COcrWord_USE
#define DEF___FIVectorView_1_Windows__CMedia__COcr__COcrWord_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("805a60c7-df4f-527c-86b2-e29e439a83d2"))
IVectorView<ABI::Windows::Media::Ocr::OcrWord*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Ocr::OcrWord*, ABI::Windows::Media::Ocr::IOcrWord*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Ocr.OcrWord>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Ocr::OcrWord*> __FIVectorView_1_Windows__CMedia__COcr__COcrWord_t;
#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__COcr__COcrWord_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__COcr__COcrWord_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_double_USE
#define DEF___FIReference_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f2d6c29-5473-5f3e-92e7-96572bb990e2"))
IReference<double> : IReference_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<double> __FIReference_1_double_t;
#define __FIReference_1_double ABI::Windows::Foundation::__FIReference_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_double_USE */


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
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class SoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface ISoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap ABI::Windows::Graphics::Imaging::ISoftwareBitmap

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                class OcrEngine;
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Media.Ocr.IOcrEngine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrEngine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrEngine[] = L"Windows.Media.Ocr.IOcrEngine";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                MIDL_INTERFACE("5a14bc41-5b76-3140-b680-8825562683ac")
                IOcrEngine : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RecognizeAsync(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap* bitmap,
                        __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RecognizerLanguage(
                        ABI::Windows::Globalization::ILanguage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOcrEngine = __uuidof(IOcrEngine);
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrEngine;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Ocr.IOcrEngineStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrEngine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrEngineStatics[] = L"Windows.Media.Ocr.IOcrEngineStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                MIDL_INTERFACE("5bffa85a-3384-3540-9940-699120d428a8")
                IOcrEngineStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxImageDimension(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AvailableRecognizerLanguages(
                        __FIVectorView_1_Windows__CGlobalization__CLanguage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsLanguageSupported(
                        ABI::Windows::Globalization::ILanguage* language,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCreateFromLanguage(
                        ABI::Windows::Globalization::ILanguage* language,
                        ABI::Windows::Media::Ocr::IOcrEngine** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCreateFromUserProfileLanguages(
                        ABI::Windows::Media::Ocr::IOcrEngine** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOcrEngineStatics = __uuidof(IOcrEngineStatics);
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Ocr.IOcrLine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrLine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrLine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrLine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrLine[] = L"Windows.Media.Ocr.IOcrLine";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                MIDL_INTERFACE("0043a16f-e31f-3a24-899c-d444bd088124")
                IOcrLine : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Words(
                        __FIVectorView_1_Windows__CMedia__COcr__COcrWord** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOcrLine = __uuidof(IOcrLine);
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrLine;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrLine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Ocr.IOcrResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrResult[] = L"Windows.Media.Ocr.IOcrResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                MIDL_INTERFACE("9bd235b2-175b-3d6a-92e2-388c206e2f63")
                IOcrResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Lines(
                        __FIVectorView_1_Windows__CMedia__COcr__COcrLine** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TextAngle(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOcrResult = __uuidof(IOcrResult);
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Ocr.IOcrWord
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrWord
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrWord_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrWord_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrWord[] = L"Windows.Media.Ocr.IOcrWord";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Ocr {
                MIDL_INTERFACE("3c2a477a-5cd9-3525-ba2a-23d1e0a68a1d")
                IOcrWord : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BoundingRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOcrWord = __uuidof(IOcrWord);
            } /* Ocr */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrWord;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrWord_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Ocr.OcrEngine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Ocr.IOcrEngineStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Ocr.IOcrEngine ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Ocr_OcrEngine_DEFINED
#define RUNTIMECLASS_Windows_Media_Ocr_OcrEngine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Ocr_OcrEngine[] = L"Windows.Media.Ocr.OcrEngine";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Ocr.OcrLine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Ocr.IOcrLine ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Ocr_OcrLine_DEFINED
#define RUNTIMECLASS_Windows_Media_Ocr_OcrLine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Ocr_OcrLine[] = L"Windows.Media.Ocr.OcrLine";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Ocr.OcrResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Ocr.IOcrResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Ocr_OcrResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Ocr_OcrResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Ocr_OcrResult[] = L"Windows.Media.Ocr.OcrResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Ocr.OcrWord
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Ocr.IOcrWord ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Ocr_OcrWord_DEFINED
#define RUNTIMECLASS_Windows_Media_Ocr_OcrWord_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Ocr_OcrWord[] = L"Windows.Media.Ocr.OcrWord";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_COcr_CIOcrEngine __x_ABI_CWindows_CMedia_COcr_CIOcrEngine;

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics;

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrLine_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrLine_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_COcr_CIOcrLine __x_ABI_CWindows_CMedia_COcr_CIOcrLine;

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrLine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_COcr_CIOcrResult __x_ABI_CWindows_CMedia_COcr_CIOcrResult;

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_COcr_CIOcrWord_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrWord_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_COcr_CIOcrWord __x_ABI_CWindows_CMedia_COcr_CIOcrWord;

#endif // ____x_ABI_CWindows_CMedia_COcr_CIOcrWord_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* This,
        __x_ABI_CWindows_CMedia_COcr_CIOcrResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult* This,
        __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__COcr__COcrResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguage __x_ABI_CWindows_CGlobalization_CILanguage;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGlobalization__CLanguage __FIIterator_1_Windows__CGlobalization__CLanguage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGlobalization__CLanguage;

typedef struct __FIIterator_1_Windows__CGlobalization__CLanguageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGlobalization__CLanguage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGlobalization__CLanguage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        __x_ABI_CWindows_CGlobalization_CILanguage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGlobalization__CLanguage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGlobalization_CILanguage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGlobalization__CLanguageVtbl;

interface __FIIterator_1_Windows__CGlobalization__CLanguage
{
    CONST_VTBL struct __FIIterator_1_Windows__CGlobalization__CLanguageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGlobalization__CLanguage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGlobalization__CLanguage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGlobalization__CLanguage __FIIterable_1_Windows__CGlobalization__CLanguage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGlobalization__CLanguage;

typedef struct __FIIterable_1_Windows__CGlobalization__CLanguageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGlobalization__CLanguage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGlobalization__CLanguage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGlobalization__CLanguage* This,
        __FIIterator_1_Windows__CGlobalization__CLanguage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGlobalization__CLanguageVtbl;

interface __FIIterable_1_Windows__CGlobalization__CLanguage
{
    CONST_VTBL struct __FIIterable_1_Windows__CGlobalization__CLanguageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGlobalization__CLanguage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGlobalization__CLanguage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__COcr__COcrLine __FIIterator_1_Windows__CMedia__COcr__COcrLine;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__COcr__COcrLine;

typedef struct __FIIterator_1_Windows__CMedia__COcr__COcrLineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This,
        __x_ABI_CWindows_CMedia_COcr_CIOcrLine** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__COcr__COcrLine* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_COcr_CIOcrLine** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__COcr__COcrLineVtbl;

interface __FIIterator_1_Windows__CMedia__COcr__COcrLine
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__COcr__COcrLineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__COcr__COcrLine_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__COcr__COcrLine __FIIterable_1_Windows__CMedia__COcr__COcrLine;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__COcr__COcrLine;

typedef struct __FIIterable_1_Windows__CMedia__COcr__COcrLineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__COcr__COcrLine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__COcr__COcrLine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__COcr__COcrLine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__COcr__COcrLine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__COcr__COcrLine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__COcr__COcrLine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__COcr__COcrLine* This,
        __FIIterator_1_Windows__CMedia__COcr__COcrLine** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__COcr__COcrLineVtbl;

interface __FIIterable_1_Windows__CMedia__COcr__COcrLine
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__COcr__COcrLineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__COcr__COcrLine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__COcr__COcrLine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__COcr__COcrLine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__COcr__COcrLine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__COcr__COcrLine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__COcr__COcrLine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__COcr__COcrLine_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__COcr__COcrWord __FIIterator_1_Windows__CMedia__COcr__COcrWord;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__COcr__COcrWord;

typedef struct __FIIterator_1_Windows__CMedia__COcr__COcrWordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This,
        __x_ABI_CWindows_CMedia_COcr_CIOcrWord** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__COcr__COcrWord* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_COcr_CIOcrWord** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__COcr__COcrWordVtbl;

interface __FIIterator_1_Windows__CMedia__COcr__COcrWord
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__COcr__COcrWordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__COcr__COcrWord_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__COcr__COcrWord __FIIterable_1_Windows__CMedia__COcr__COcrWord;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__COcr__COcrWord;

typedef struct __FIIterable_1_Windows__CMedia__COcr__COcrWordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__COcr__COcrWord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__COcr__COcrWord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__COcr__COcrWord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__COcr__COcrWord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__COcr__COcrWord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__COcr__COcrWord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__COcr__COcrWord* This,
        __FIIterator_1_Windows__CMedia__COcr__COcrWord** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__COcr__COcrWordVtbl;

interface __FIIterable_1_Windows__CMedia__COcr__COcrWord
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__COcr__COcrWordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__COcr__COcrWord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__COcr__COcrWord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__COcr__COcrWord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__COcr__COcrWord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__COcr__COcrWord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__COcr__COcrWord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__COcr__COcrWord_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGlobalization__CLanguage __FIVectorView_1_Windows__CGlobalization__CLanguage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGlobalization__CLanguage;

typedef struct __FIVectorView_1_Windows__CGlobalization__CLanguageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        UINT32 index,
        __x_ABI_CWindows_CGlobalization_CILanguage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        __x_ABI_CWindows_CGlobalization_CILanguage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGlobalization__CLanguage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGlobalization_CILanguage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGlobalization__CLanguageVtbl;

interface __FIVectorView_1_Windows__CGlobalization__CLanguage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGlobalization__CLanguageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGlobalization__CLanguage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGlobalization__CLanguage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__COcr__COcrLine __FIVectorView_1_Windows__CMedia__COcr__COcrLine;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__COcr__COcrLine;

typedef struct __FIVectorView_1_Windows__CMedia__COcr__COcrLineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_COcr_CIOcrLine** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This,
        __x_ABI_CWindows_CMedia_COcr_CIOcrLine* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__COcr__COcrLine* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_COcr_CIOcrLine** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__COcr__COcrLineVtbl;

interface __FIVectorView_1_Windows__CMedia__COcr__COcrLine
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__COcr__COcrLineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrLine_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__COcr__COcrLine_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__COcr__COcrWord __FIVectorView_1_Windows__CMedia__COcr__COcrWord;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__COcr__COcrWord;

typedef struct __FIVectorView_1_Windows__CMedia__COcr__COcrWordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_COcr_CIOcrWord** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This,
        __x_ABI_CWindows_CMedia_COcr_CIOcrWord* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__COcr__COcrWord* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_COcr_CIOcrWord** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__COcr__COcrWordVtbl;

interface __FIVectorView_1_Windows__CMedia__COcr__COcrWord
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__COcr__COcrWordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__COcr__COcrWord_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__COcr__COcrWord_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_double_INTERFACE_DEFINED__)
#define ____FIReference_1_double_INTERFACE_DEFINED__

typedef interface __FIReference_1_double __FIReference_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_double;

typedef struct __FIReference_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_double* This,
        DOUBLE* result);

    END_INTERFACE
} __FIReference_1_doubleVtbl;

interface __FIReference_1_double
{
    CONST_VTBL struct __FIReference_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_double_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_double_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

/*
 *
 * Interface Windows.Media.Ocr.IOcrEngine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrEngine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrEngine[] = L"Windows.Media.Ocr.IOcrEngine";
typedef struct __x_ABI_CWindows_CMedia_COcr_CIOcrEngineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RecognizeAsync)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngine* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* bitmap,
        __FIAsyncOperation_1_Windows__CMedia__COcr__COcrResult** result);
    HRESULT (STDMETHODCALLTYPE* get_RecognizerLanguage)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngine* This,
        __x_ABI_CWindows_CGlobalization_CILanguage** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_COcr_CIOcrEngineVtbl;

interface __x_ABI_CWindows_CMedia_COcr_CIOcrEngine
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_COcr_CIOcrEngineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine_RecognizeAsync(This, bitmap, result) \
    ((This)->lpVtbl->RecognizeAsync(This, bitmap, result))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngine_get_RecognizerLanguage(This, value) \
    ((This)->lpVtbl->get_RecognizerLanguage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrEngine;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrEngine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Ocr.IOcrEngineStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrEngine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrEngineStatics[] = L"Windows.Media.Ocr.IOcrEngineStatics";
typedef struct __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxImageDimension)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AvailableRecognizerLanguages)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        __FIVectorView_1_Windows__CGlobalization__CLanguage** value);
    HRESULT (STDMETHODCALLTYPE* IsLanguageSupported)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        __x_ABI_CWindows_CGlobalization_CILanguage* language,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TryCreateFromLanguage)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        __x_ABI_CWindows_CGlobalization_CILanguage* language,
        __x_ABI_CWindows_CMedia_COcr_CIOcrEngine** result);
    HRESULT (STDMETHODCALLTYPE* TryCreateFromUserProfileLanguages)(__x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics* This,
        __x_ABI_CWindows_CMedia_COcr_CIOcrEngine** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStaticsVtbl;

interface __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_get_MaxImageDimension(This, value) \
    ((This)->lpVtbl->get_MaxImageDimension(This, value))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_get_AvailableRecognizerLanguages(This, value) \
    ((This)->lpVtbl->get_AvailableRecognizerLanguages(This, value))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_IsLanguageSupported(This, language, result) \
    ((This)->lpVtbl->IsLanguageSupported(This, language, result))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_TryCreateFromLanguage(This, language, result) \
    ((This)->lpVtbl->TryCreateFromLanguage(This, language, result))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_TryCreateFromUserProfileLanguages(This, result) \
    ((This)->lpVtbl->TryCreateFromUserProfileLanguages(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrEngineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Ocr.IOcrLine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrLine
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrLine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrLine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrLine[] = L"Windows.Media.Ocr.IOcrLine";
typedef struct __x_ABI_CWindows_CMedia_COcr_CIOcrLineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_COcr_CIOcrLine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_COcr_CIOcrLine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_COcr_CIOcrLine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_COcr_CIOcrLine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_COcr_CIOcrLine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_COcr_CIOcrLine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Words)(__x_ABI_CWindows_CMedia_COcr_CIOcrLine* This,
        __FIVectorView_1_Windows__CMedia__COcr__COcrWord** value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CMedia_COcr_CIOcrLine* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_COcr_CIOcrLineVtbl;

interface __x_ABI_CWindows_CMedia_COcr_CIOcrLine
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_COcr_CIOcrLineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine_get_Words(This, value) \
    ((This)->lpVtbl->get_Words(This, value))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrLine_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrLine;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrLine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Ocr.IOcrResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrResult[] = L"Windows.Media.Ocr.IOcrResult";
typedef struct __x_ABI_CWindows_CMedia_COcr_CIOcrResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Lines)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This,
        __FIVectorView_1_Windows__CMedia__COcr__COcrLine** value);
    HRESULT (STDMETHODCALLTYPE* get_TextAngle)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CMedia_COcr_CIOcrResult* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_COcr_CIOcrResultVtbl;

interface __x_ABI_CWindows_CMedia_COcr_CIOcrResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_COcr_CIOcrResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_get_Lines(This, value) \
    ((This)->lpVtbl->get_Lines(This, value))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_get_TextAngle(This, value) \
    ((This)->lpVtbl->get_TextAngle(This, value))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrResult_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Ocr.IOcrWord
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Ocr.OcrWord
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrWord_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_COcr_CIOcrWord_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Ocr_IOcrWord[] = L"Windows.Media.Ocr.IOcrWord";
typedef struct __x_ABI_CWindows_CMedia_COcr_CIOcrWordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_COcr_CIOcrWord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_COcr_CIOcrWord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_COcr_CIOcrWord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_COcr_CIOcrWord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_COcr_CIOcrWord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_COcr_CIOcrWord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BoundingRect)(__x_ABI_CWindows_CMedia_COcr_CIOcrWord* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CMedia_COcr_CIOcrWord* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_COcr_CIOcrWordVtbl;

interface __x_ABI_CWindows_CMedia_COcr_CIOcrWord
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_COcr_CIOcrWordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord_get_BoundingRect(This, value) \
    ((This)->lpVtbl->get_BoundingRect(This, value))

#define __x_ABI_CWindows_CMedia_COcr_CIOcrWord_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_COcr_CIOcrWord;
#endif /* !defined(____x_ABI_CWindows_CMedia_COcr_CIOcrWord_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Ocr.OcrEngine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Ocr.IOcrEngineStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Ocr.IOcrEngine ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Ocr_OcrEngine_DEFINED
#define RUNTIMECLASS_Windows_Media_Ocr_OcrEngine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Ocr_OcrEngine[] = L"Windows.Media.Ocr.OcrEngine";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Ocr.OcrLine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Ocr.IOcrLine ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Ocr_OcrLine_DEFINED
#define RUNTIMECLASS_Windows_Media_Ocr_OcrLine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Ocr_OcrLine[] = L"Windows.Media.Ocr.OcrLine";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Ocr.OcrResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Ocr.IOcrResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Ocr_OcrResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Ocr_OcrResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Ocr_OcrResult[] = L"Windows.Media.Ocr.OcrResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Ocr.OcrWord
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Ocr.IOcrWord ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Ocr_OcrWord_DEFINED
#define RUNTIMECLASS_Windows_Media_Ocr_OcrWord_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Ocr_OcrWord[] = L"Windows.Media.Ocr.OcrWord";
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
#endif // __windows2Emedia2Eocr_p_h__

#endif // __windows2Emedia2Eocr_h__
