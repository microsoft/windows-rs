
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
#ifndef __windows2Emedia2Espeechsynthesis_h__
#define __windows2Emedia2Espeechsynthesis_h__
#ifndef __windows2Emedia2Espeechsynthesis_p_h__
#define __windows2Emedia2Espeechsynthesis_p_h__


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

#if !defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)
#define WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Media.h"
#include "Windows.Media.Core.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface IInstalledVoicesStatic;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic ABI::Windows::Media::SpeechSynthesis::IInstalledVoicesStatic

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface IInstalledVoicesStatic2;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2 ABI::Windows::Media::SpeechSynthesis::IInstalledVoicesStatic2

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface ISpeechSynthesisStream;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesisStream

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface ISpeechSynthesizer;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesizer

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface ISpeechSynthesizer2;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2 ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesizer2

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface ISpeechSynthesizerOptions;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesizerOptions

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface ISpeechSynthesizerOptions2;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2 ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesizerOptions2

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface ISpeechSynthesizerOptions3;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3 ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesizerOptions3

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                interface IVoiceInformation;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation ABI::Windows::Media::SpeechSynthesis::IVoiceInformation

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_FWD_DEFINED__

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


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                class SpeechSynthesisStream;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("df9d48ad-9cea-560c-9edc-cb8852cb55e3"))
IAsyncOperation<ABI::Windows::Media::SpeechSynthesis::SpeechSynthesisStream*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechSynthesis::SpeechSynthesisStream*, ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesisStream*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.SpeechSynthesis.SpeechSynthesisStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::SpeechSynthesis::SpeechSynthesisStream*> __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_t;
#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c972b996-6165-50d4-af60-a8c3df51d092"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::SpeechSynthesis::SpeechSynthesisStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechSynthesis::SpeechSynthesisStream*, ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesisStream*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.SpeechSynthesis.SpeechSynthesisStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::SpeechSynthesis::SpeechSynthesisStream*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaMarker;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaMarker ABI::Windows::Media::IMediaMarker

#endif // ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CIMediaMarker_USE
#define DEF___FIIterator_1_Windows__CMedia__CIMediaMarker_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f464661e-88bc-5cea-93cd-0c123f17d258"))
IIterator<ABI::Windows::Media::IMediaMarker*> : IIterator_impl<ABI::Windows::Media::IMediaMarker*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.IMediaMarker>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::IMediaMarker*> __FIIterator_1_Windows__CMedia__CIMediaMarker_t;
#define __FIIterator_1_Windows__CMedia__CIMediaMarker ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CIMediaMarker_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CIMediaMarker_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CIMediaMarker_USE
#define DEF___FIIterable_1_Windows__CMedia__CIMediaMarker_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a1c0a397-0364-5e4c-9dca-7cd7011bd114"))
IIterable<ABI::Windows::Media::IMediaMarker*> : IIterable_impl<ABI::Windows::Media::IMediaMarker*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.IMediaMarker>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::IMediaMarker*> __FIIterable_1_Windows__CMedia__CIMediaMarker_t;
#define __FIIterable_1_Windows__CMedia__CIMediaMarker ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CIMediaMarker_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CIMediaMarker_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                class VoiceInformation;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE
#define DEF___FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("12d40a27-ae8d-5fb0-8fed-00165d59c6ab"))
IIterator<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*, ABI::Windows::Media::SpeechSynthesis::IVoiceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.SpeechSynthesis.VoiceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*> __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_t;
#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE
#define DEF___FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3c33bb52-bd98-5c8c-adee-ee8da0628efc"))
IIterable<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*, ABI::Windows::Media::SpeechSynthesis::IVoiceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.SpeechSynthesis.VoiceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*> __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_t;
#define __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CIMediaMarker_USE
#define DEF___FIVectorView_1_Windows__CMedia__CIMediaMarker_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b543562c-02b1-5824-80a8-9854130cdadd"))
IVectorView<ABI::Windows::Media::IMediaMarker*> : IVectorView_impl<ABI::Windows::Media::IMediaMarker*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.IMediaMarker>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::IMediaMarker*> __FIVectorView_1_Windows__CMedia__CIMediaMarker_t;
#define __FIVectorView_1_Windows__CMedia__CIMediaMarker ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CIMediaMarker_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CIMediaMarker_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE
#define DEF___FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ee8d63ce-51ac-5984-891b-d232fa7f6453"))
IVectorView<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*, ABI::Windows::Media::SpeechSynthesis::IVoiceInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.SpeechSynthesis.VoiceInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::SpeechSynthesis::VoiceInformation*> __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_t;
#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface ITimedMetadataTrackProvider;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider ABI::Windows::Media::Core::ITimedMetadataTrackProvider

#endif // ____x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IContentTypeProvider;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider ABI::Windows::Storage::Streams::IContentTypeProvider

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamWithContentType;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                typedef enum SpeechAppendedSilence : int SpeechAppendedSilence;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                typedef enum SpeechPunctuationSilence : int SpeechPunctuationSilence;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                typedef enum VoiceGender : int VoiceGender;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                class SpeechSynthesizerOptions;
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.SpeechSynthesis.SpeechAppendedSilence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                enum SpeechAppendedSilence : int
                {
                    SpeechAppendedSilence_Default = 0,
                    SpeechAppendedSilence_Min = 1,
                };
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Media.SpeechSynthesis.SpeechPunctuationSilence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                enum SpeechPunctuationSilence : int
                {
                    SpeechPunctuationSilence_Default = 0,
                    SpeechPunctuationSilence_Min = 1,
                };
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Media.SpeechSynthesis.VoiceGender
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                enum VoiceGender : int
                {
                    VoiceGender_Male = 0,
                    VoiceGender_Female = 1,
                };
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.IInstalledVoicesStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_IInstalledVoicesStatic[] = L"Windows.Media.SpeechSynthesis.IInstalledVoicesStatic";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("7d526ecc-7533-4c3f-85be-888c2baeebdc")
                IInstalledVoicesStatic : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AllVoices(
                        __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultVoice(
                        ABI::Windows::Media::SpeechSynthesis::IVoiceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInstalledVoicesStatic = __uuidof(IInstalledVoicesStatic);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_IInstalledVoicesStatic2[] = L"Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("64255f2e-358d-4058-be9a-fd3fcb423530")
                IInstalledVoicesStatic2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TrySetDefaultVoiceAsync(
                        ABI::Windows::Media::SpeechSynthesis::IVoiceInformation* voice,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInstalledVoicesStatic2 = __uuidof(IInstalledVoicesStatic2);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesisStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesisStream
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IRandomAccessStreamWithContentType
 *     Windows.Storage.Streams.IRandomAccessStream
 *     Windows.Foundation.IClosable
 *     Windows.Storage.Streams.IInputStream
 *     Windows.Storage.Streams.IOutputStream
 *     Windows.Storage.Streams.IContentTypeProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesisStream[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesisStream";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("83e46e93-244c-4622-ba0b-6229c4d0d65d")
                ISpeechSynthesisStream : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Markers(
                        __FIVectorView_1_Windows__CMedia__CIMediaMarker** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechSynthesisStream = __uuidof(ISpeechSynthesisStream);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizer[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizer";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("ce9f7c76-97f4-4ced-ad68-d51c458e45c6")
                ISpeechSynthesizer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SynthesizeTextToStreamAsync(
                        HSTRING text,
                        __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SynthesizeSsmlToStreamAsync(
                        HSTRING Ssml,
                        __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Voice(
                        ABI::Windows::Media::SpeechSynthesis::IVoiceInformation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Voice(
                        ABI::Windows::Media::SpeechSynthesis::IVoiceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechSynthesizer = __uuidof(ISpeechSynthesizer);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizer2[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizer2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("a7c5ecb2-4339-4d6a-bbf8-c7a4f1544c2e")
                ISpeechSynthesizer2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Options(
                        ABI::Windows::Media::SpeechSynthesis::ISpeechSynthesizerOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechSynthesizer2 = __uuidof(ISpeechSynthesizer2);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizerOptions[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("a0e23871-cc3d-43c9-91b1-ee185324d83d")
                ISpeechSynthesizerOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeWordBoundaryMetadata(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IncludeWordBoundaryMetadata(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeSentenceBoundaryMetadata(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IncludeSentenceBoundaryMetadata(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechSynthesizerOptions = __uuidof(ISpeechSynthesizerOptions);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizerOptions2[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("1cbef60e-119c-4bed-b118-d250c3a25793")
                ISpeechSynthesizerOptions2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AudioVolume(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AudioVolume(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SpeakingRate(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SpeakingRate(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AudioPitch(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AudioPitch(
                        DOUBLE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechSynthesizerOptions2 = __uuidof(ISpeechSynthesizerOptions2);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizerOptions3[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("401ed877-902c-4814-a582-a5d0c0769fa8")
                ISpeechSynthesizerOptions3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppendedSilence(
                        ABI::Windows::Media::SpeechSynthesis::SpeechAppendedSilence* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AppendedSilence(
                        ABI::Windows::Media::SpeechSynthesis::SpeechAppendedSilence value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PunctuationSilence(
                        ABI::Windows::Media::SpeechSynthesis::SpeechPunctuationSilence* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PunctuationSilence(
                        ABI::Windows::Media::SpeechSynthesis::SpeechPunctuationSilence value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpeechSynthesizerOptions3 = __uuidof(ISpeechSynthesizerOptions3);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.IVoiceInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.VoiceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_IVoiceInformation[] = L"Windows.Media.SpeechSynthesis.IVoiceInformation";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechSynthesis {
                MIDL_INTERFACE("b127d6a4-1291-4604-aa9c-83134083352c")
                IVoiceInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Gender(
                        ABI::Windows::Media::SpeechSynthesis::VoiceGender* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoiceInformation = __uuidof(IVoiceInformation);
            } /* SpeechSynthesis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechSynthesis.SpeechSynthesisStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesisStream ** Default Interface **
 *    Windows.Storage.Streams.IRandomAccessStreamWithContentType
 *    Windows.Storage.Streams.IContentTypeProvider
 *    Windows.Storage.Streams.IRandomAccessStream
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *    Windows.Media.Core.ITimedMetadataTrackProvider
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesisStream_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesisStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechSynthesis_SpeechSynthesisStream[] = L"Windows.Media.SpeechSynthesis.SpeechSynthesisStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.SpeechSynthesis.IInstalledVoicesStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizer ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizer2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesizer_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechSynthesis_SpeechSynthesizer[] = L"Windows.Media.SpeechSynthesis.SpeechSynthesizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions ** Default Interface **
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions2
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesizerOptions_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesizerOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechSynthesis_SpeechSynthesizerOptions[] = L"Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.SpeechSynthesis.VoiceInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechSynthesis.IVoiceInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechSynthesis_VoiceInformation_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechSynthesis_VoiceInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechSynthesis_VoiceInformation[] = L"Windows.Media.SpeechSynthesis.VoiceInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2 __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2 __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2 __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3 __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation;

#endif // ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStreamVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* This,
        __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaMarker __x_ABI_CWindows_CMedia_CIMediaMarker;

#endif // ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CIMediaMarker __FIIterator_1_Windows__CMedia__CIMediaMarker;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CIMediaMarker;

typedef struct __FIIterator_1_Windows__CMedia__CIMediaMarkerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        __x_ABI_CWindows_CMedia_CIMediaMarker** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CIMediaMarker** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CIMediaMarkerVtbl;

interface __FIIterator_1_Windows__CMedia__CIMediaMarker
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CIMediaMarkerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CIMediaMarker __FIIterable_1_Windows__CMedia__CIMediaMarker;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CIMediaMarker;

typedef struct __FIIterable_1_Windows__CMedia__CIMediaMarkerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        __FIIterator_1_Windows__CMedia__CIMediaMarker** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CIMediaMarkerVtbl;

interface __FIIterable_1_Windows__CMedia__CIMediaMarker
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CIMediaMarkerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation;

typedef struct __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl;

interface __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation;

typedef struct __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        __FIIterator_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl;

interface __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CIMediaMarker __FIVectorView_1_Windows__CMedia__CIMediaMarker;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CIMediaMarker;

typedef struct __FIVectorView_1_Windows__CMedia__CIMediaMarkerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CIMediaMarker** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        __x_ABI_CWindows_CMedia_CIMediaMarker* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CIMediaMarker** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CIMediaMarkerVtbl;

interface __FIVectorView_1_Windows__CMedia__CIMediaMarker
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CIMediaMarkerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation;

typedef struct __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl;

interface __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider __x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider;

#endif // ____x_ABI_CWindows_CMedia_CCore_CITimedMetadataTrackProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechAppendedSilence __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechAppendedSilence;

typedef enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechPunctuationSilence __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechPunctuationSilence;

typedef enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CVoiceGender __x_ABI_CWindows_CMedia_CSpeechSynthesis_CVoiceGender;

/*
 *
 * Struct Windows.Media.SpeechSynthesis.SpeechAppendedSilence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechAppendedSilence
{
    SpeechAppendedSilence_Default = 0,
    SpeechAppendedSilence_Min = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Media.SpeechSynthesis.SpeechPunctuationSilence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechPunctuationSilence
{
    SpeechPunctuationSilence_Default = 0,
    SpeechPunctuationSilence_Min = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Media.SpeechSynthesis.VoiceGender
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CVoiceGender
{
    VoiceGender_Male = 0,
    VoiceGender_Female = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.IInstalledVoicesStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_IInstalledVoicesStatic[] = L"Windows.Media.SpeechSynthesis.IInstalledVoicesStatic";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStaticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllVoices)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic* This,
        __FIVectorView_1_Windows__CMedia__CSpeechSynthesis__CVoiceInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultVoice)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic* This,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStaticVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStaticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_get_AllVoices(This, value) \
    ((This)->lpVtbl->get_AllVoices(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_get_DefaultVoice(This, value) \
    ((This)->lpVtbl->get_DefaultVoice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_IInstalledVoicesStatic2[] = L"Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TrySetDefaultVoiceAsync)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2* This,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* voice,
        __FIAsyncOperation_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2Vtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_TrySetDefaultVoiceAsync(This, voice, result) \
    ((This)->lpVtbl->TrySetDefaultVoiceAsync(This, voice, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIInstalledVoicesStatic2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesisStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesisStream
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IRandomAccessStreamWithContentType
 *     Windows.Storage.Streams.IRandomAccessStream
 *     Windows.Foundation.IClosable
 *     Windows.Storage.Streams.IInputStream
 *     Windows.Storage.Streams.IOutputStream
 *     Windows.Storage.Streams.IContentTypeProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesisStream[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesisStream";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Markers)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream* This,
        __FIVectorView_1_Windows__CMedia__CIMediaMarker** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStreamVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_get_Markers(This, value) \
    ((This)->lpVtbl->get_Markers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesisStream_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizer[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizer";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SynthesizeTextToStreamAsync)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This,
        HSTRING text,
        __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream** operation);
    HRESULT (STDMETHODCALLTYPE* SynthesizeSsmlToStreamAsync)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This,
        HSTRING Ssml,
        __FIAsyncOperation_1_Windows__CMedia__CSpeechSynthesis__CSpeechSynthesisStream** operation);
    HRESULT (STDMETHODCALLTYPE* put_Voice)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* value);
    HRESULT (STDMETHODCALLTYPE* get_Voice)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer* This,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_SynthesizeTextToStreamAsync(This, text, operation) \
    ((This)->lpVtbl->SynthesizeTextToStreamAsync(This, text, operation))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_SynthesizeSsmlToStreamAsync(This, Ssml, operation) \
    ((This)->lpVtbl->SynthesizeSsmlToStreamAsync(This, Ssml, operation))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_put_Voice(This, value) \
    ((This)->lpVtbl->put_Voice(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_get_Voice(This, value) \
    ((This)->lpVtbl->get_Voice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizer2[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizer2";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2* This,
        __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2Vtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizerOptions[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IncludeWordBoundaryMetadata)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeWordBoundaryMetadata)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeSentenceBoundaryMetadata)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeSentenceBoundaryMetadata)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptionsVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_get_IncludeWordBoundaryMetadata(This, value) \
    ((This)->lpVtbl->get_IncludeWordBoundaryMetadata(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_put_IncludeWordBoundaryMetadata(This, value) \
    ((This)->lpVtbl->put_IncludeWordBoundaryMetadata(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_get_IncludeSentenceBoundaryMetadata(This, value) \
    ((This)->lpVtbl->get_IncludeSentenceBoundaryMetadata(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_put_IncludeSentenceBoundaryMetadata(This, value) \
    ((This)->lpVtbl->put_IncludeSentenceBoundaryMetadata(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizerOptions2[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions2";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AudioVolume)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_AudioVolume)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_SpeakingRate)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_SpeakingRate)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_AudioPitch)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_AudioPitch)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2Vtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_get_AudioVolume(This, value) \
    ((This)->lpVtbl->get_AudioVolume(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_put_AudioVolume(This, value) \
    ((This)->lpVtbl->put_AudioVolume(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_get_SpeakingRate(This, value) \
    ((This)->lpVtbl->get_SpeakingRate(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_put_SpeakingRate(This, value) \
    ((This)->lpVtbl->put_SpeakingRate(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_get_AudioPitch(This, value) \
    ((This)->lpVtbl->get_AudioPitch(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_put_AudioPitch(This, value) \
    ((This)->lpVtbl->put_AudioPitch(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_ISpeechSynthesizerOptions3[] = L"Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions3";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppendedSilence)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This,
        enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechAppendedSilence* value);
    HRESULT (STDMETHODCALLTYPE* put_AppendedSilence)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This,
        enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechAppendedSilence value);
    HRESULT (STDMETHODCALLTYPE* get_PunctuationSilence)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This,
        enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechPunctuationSilence* value);
    HRESULT (STDMETHODCALLTYPE* put_PunctuationSilence)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3* This,
        enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CSpeechPunctuationSilence value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3Vtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_get_AppendedSilence(This, value) \
    ((This)->lpVtbl->get_AppendedSilence(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_put_AppendedSilence(This, value) \
    ((This)->lpVtbl->put_AppendedSilence(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_get_PunctuationSilence(This, value) \
    ((This)->lpVtbl->get_PunctuationSilence(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_put_PunctuationSilence(This, value) \
    ((This)->lpVtbl->put_PunctuationSilence(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CISpeechSynthesizerOptions3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.SpeechSynthesis.IVoiceInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SpeechSynthesis.VoiceInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_SpeechSynthesis_IVoiceInformation[] = L"Windows.Media.SpeechSynthesis.IVoiceInformation";
typedef struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Gender)(__x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation* This,
        enum __x_ABI_CWindows_CMedia_CSpeechSynthesis_CVoiceGender* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformationVtbl;

interface __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_get_Gender(This, value) \
    ((This)->lpVtbl->get_Gender(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation;
#endif /* !defined(____x_ABI_CWindows_CMedia_CSpeechSynthesis_CIVoiceInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechSynthesis.SpeechSynthesisStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesisStream ** Default Interface **
 *    Windows.Storage.Streams.IRandomAccessStreamWithContentType
 *    Windows.Storage.Streams.IContentTypeProvider
 *    Windows.Storage.Streams.IRandomAccessStream
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *    Windows.Media.Core.ITimedMetadataTrackProvider
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesisStream_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesisStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechSynthesis_SpeechSynthesisStream[] = L"Windows.Media.SpeechSynthesis.SpeechSynthesisStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechSynthesis.SpeechSynthesizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.SpeechSynthesis.IInstalledVoicesStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizer ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizer2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesizer_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechSynthesis_SpeechSynthesizer[] = L"Windows.Media.SpeechSynthesis.SpeechSynthesizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions ** Default Interface **
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions2
 *    Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesizerOptions_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechSynthesis_SpeechSynthesizerOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechSynthesis_SpeechSynthesizerOptions[] = L"Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.SpeechSynthesis.VoiceInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.SpeechSynthesis.IVoiceInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SpeechSynthesis_VoiceInformation_DEFINED
#define RUNTIMECLASS_Windows_Media_SpeechSynthesis_VoiceInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SpeechSynthesis_VoiceInformation[] = L"Windows.Media.SpeechSynthesis.VoiceInformation";
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
#endif // __windows2Emedia2Espeechsynthesis_p_h__

#endif // __windows2Emedia2Espeechsynthesis_h__
