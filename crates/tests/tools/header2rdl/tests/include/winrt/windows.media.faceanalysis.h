
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
#ifndef __windows2Emedia2Efaceanalysis_h__
#define __windows2Emedia2Efaceanalysis_h__
#ifndef __windows2Emedia2Efaceanalysis_p_h__
#define __windows2Emedia2Efaceanalysis_p_h__


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
#include "Windows.Graphics.Imaging.h"
#include "Windows.Media.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                interface IDetectedFace;
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace ABI::Windows::Media::FaceAnalysis::IDetectedFace

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                interface IFaceDetector;
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector ABI::Windows::Media::FaceAnalysis::IFaceDetector

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                interface IFaceDetectorStatics;
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics ABI::Windows::Media::FaceAnalysis::IFaceDetectorStatics

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                interface IFaceTracker;
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker ABI::Windows::Media::FaceAnalysis::IFaceTracker

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                interface IFaceTrackerStatics;
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics ABI::Windows::Media::FaceAnalysis::IFaceTrackerStatics

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                class DetectedFace;
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#define DEF___FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a0cf090e-0340-531f-898b-c21eafb9f4ae"))
IIterator<ABI::Windows::Media::FaceAnalysis::DetectedFace*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::FaceAnalysis::DetectedFace*, ABI::Windows::Media::FaceAnalysis::IDetectedFace*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.FaceAnalysis.DetectedFace>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::FaceAnalysis::DetectedFace*> __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t;
#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#define DEF___FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("25347323-3556-5cbb-9855-2b5856437f4d"))
IIterable<ABI::Windows::Media::FaceAnalysis::DetectedFace*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::FaceAnalysis::DetectedFace*, ABI::Windows::Media::FaceAnalysis::IDetectedFace*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.FaceAnalysis.DetectedFace>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::FaceAnalysis::DetectedFace*> __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t;
#define __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#define DEF___FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("39ef4411-0618-5b8d-8ea2-81c637f823f8"))
IVectorView<ABI::Windows::Media::FaceAnalysis::DetectedFace*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::FaceAnalysis::DetectedFace*, ABI::Windows::Media::FaceAnalysis::IDetectedFace*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.FaceAnalysis.DetectedFace>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::FaceAnalysis::DetectedFace*> __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t;
#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#define DEF___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0dfd8e09-73f2-505c-a796-a8f031b4ede0"))
IVector<ABI::Windows::Media::FaceAnalysis::DetectedFace*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::FaceAnalysis::DetectedFace*, ABI::Windows::Media::FaceAnalysis::IDetectedFace*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.FaceAnalysis.DetectedFace>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::FaceAnalysis::DetectedFace*> __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t;
#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#define DEF___FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("37f1d7dc-a1a4-5a94-b33b-74205a65a1ed"))
IAsyncOperation<__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace*> : IAsyncOperation_impl<__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVector`1<Windows.Media.FaceAnalysis.DetectedFace>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace*> __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t;
#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace ABI::Windows::Foundation::__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b0a53153-2015-58b3-9dd0-bdf291b856b2"))
IAsyncOperationCompletedHandler<__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace*> : IAsyncOperationCompletedHandler_impl<__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVector`1<Windows.Media.FaceAnalysis.DetectedFace>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace*> __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t;
#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                class FaceDetector;
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c0141cd2-7a65-514c-bfc4-b49e991f03eb"))
IAsyncOperation<ABI::Windows::Media::FaceAnalysis::FaceDetector*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::FaceAnalysis::FaceDetector*, ABI::Windows::Media::FaceAnalysis::IFaceDetector*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.FaceAnalysis.FaceDetector>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::FaceAnalysis::FaceDetector*> __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_t;
#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3224aec6-e785-5066-976f-79dd081b75a9"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::FaceAnalysis::FaceDetector*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::FaceAnalysis::FaceDetector*, ABI::Windows::Media::FaceAnalysis::IFaceDetector*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.FaceAnalysis.FaceDetector>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::FaceAnalysis::FaceDetector*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                class FaceTracker;
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7f32c680-e37e-539a-8993-2abbea71afda"))
IAsyncOperation<ABI::Windows::Media::FaceAnalysis::FaceTracker*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::FaceAnalysis::FaceTracker*, ABI::Windows::Media::FaceAnalysis::IFaceTracker*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.FaceAnalysis.FaceTracker>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::FaceAnalysis::FaceTracker*> __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_t;
#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("57d5163d-9e80-59d2-9366-d2b5248b2724"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::FaceAnalysis::FaceTracker*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::FaceAnalysis::FaceTracker*, ABI::Windows::Media::FaceAnalysis::IFaceTracker*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.FaceAnalysis.FaceTracker>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::FaceAnalysis::FaceTracker*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapPixelFormat : int BitmapPixelFormat;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE
#define DEF___FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7fc2e293-1084-5d45-b8b8-93e10692bcc8"))
IIterator<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat> : IIterator_impl<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Imaging.BitmapPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat> __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_t;
#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE
#define DEF___FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e924d9ed-a13e-5bdb-9ed8-65a1474dc274"))
IIterable<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat> : IIterable_impl<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Imaging.BitmapPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat> __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_t;
#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("76ac4bc2-c19c-559c-b287-1694c0dc3a0d"))
IVectorView<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat> : IVectorView_impl<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Imaging.BitmapPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Graphics::Imaging::BitmapPixelFormat> __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_t;
#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef struct BitmapBounds BitmapBounds;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef struct BitmapSize BitmapSize;
            } /* Imaging */
        } /* Graphics */
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
            class VideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoFrame ABI::Windows::Media::IVideoFrame

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

/*
 *
 * Interface Windows.Media.FaceAnalysis.IDetectedFace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.DetectedFace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IDetectedFace[] = L"Windows.Media.FaceAnalysis.IDetectedFace";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                MIDL_INTERFACE("8200d454-66bc-34df-9410-e89400195414")
                IDetectedFace : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FaceBox(
                        ABI::Windows::Graphics::Imaging::BitmapBounds* returnValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDetectedFace = __uuidof(IDetectedFace);
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.FaceAnalysis.IFaceDetector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.FaceDetector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IFaceDetector[] = L"Windows.Media.FaceAnalysis.IFaceDetector";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                MIDL_INTERFACE("16b672dc-fe6f-3117-8d95-c3f04d51630c")
                IFaceDetector : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE DetectFacesAsync(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap* image,
                        __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DetectFacesWithSearchAreaAsync(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap* image,
                        ABI::Windows::Graphics::Imaging::BitmapBounds searchArea,
                        __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinDetectableFaceSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MinDetectableFaceSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxDetectableFaceSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxDetectableFaceSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFaceDetector = __uuidof(IFaceDetector);
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.FaceAnalysis.IFaceDetectorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.FaceDetector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IFaceDetectorStatics[] = L"Windows.Media.FaceAnalysis.IFaceDetectorStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                MIDL_INTERFACE("bc042d67-9047-33f6-881b-6746c1b218b8")
                IFaceDetectorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedBitmapPixelFormats(
                        __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsBitmapPixelFormatSupported(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat bitmapPixelFormat,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* returnValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFaceDetectorStatics = __uuidof(IFaceDetectorStatics);
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.FaceAnalysis.IFaceTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.FaceTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IFaceTracker[] = L"Windows.Media.FaceAnalysis.IFaceTracker";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                MIDL_INTERFACE("6ba67d8c-a841-4420-93e6-2420a1884fcf")
                IFaceTracker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ProcessNextFrameAsync(
                        ABI::Windows::Media::IVideoFrame* videoFrame,
                        __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinDetectableFaceSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MinDetectableFaceSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxDetectableFaceSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxDetectableFaceSize(
                        ABI::Windows::Graphics::Imaging::BitmapSize value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFaceTracker = __uuidof(IFaceTracker);
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.FaceAnalysis.IFaceTrackerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.FaceTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IFaceTrackerStatics[] = L"Windows.Media.FaceAnalysis.IFaceTrackerStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace FaceAnalysis {
                MIDL_INTERFACE("e9629198-1801-3fa5-932e-31d767af6c4d")
                IFaceTrackerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedBitmapPixelFormats(
                        __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsBitmapPixelFormatSupported(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat bitmapPixelFormat,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* returnValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFaceTrackerStatics = __uuidof(IFaceTrackerStatics);
            } /* FaceAnalysis */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.FaceAnalysis.DetectedFace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.FaceAnalysis.IDetectedFace ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_FaceAnalysis_DetectedFace_DEFINED
#define RUNTIMECLASS_Windows_Media_FaceAnalysis_DetectedFace_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_FaceAnalysis_DetectedFace[] = L"Windows.Media.FaceAnalysis.DetectedFace";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.FaceAnalysis.FaceDetector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.FaceAnalysis.IFaceDetectorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.FaceAnalysis.IFaceDetector ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_FaceAnalysis_FaceDetector_DEFINED
#define RUNTIMECLASS_Windows_Media_FaceAnalysis_FaceDetector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_FaceAnalysis_FaceDetector[] = L"Windows.Media.FaceAnalysis.FaceDetector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.FaceAnalysis.FaceTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.FaceAnalysis.IFaceTrackerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.FaceAnalysis.IFaceTracker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_FaceAnalysis_FaceTracker_DEFINED
#define RUNTIMECLASS_Windows_Media_FaceAnalysis_FaceTracker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_FaceAnalysis_FaceTracker[] = L"Windows.Media.FaceAnalysis.FaceTracker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace;

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector;

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics;

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker;

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics;

#endif // ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

typedef struct __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl;

interface __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

typedef struct __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __FIIterator_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl;

interface __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

typedef struct __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl;

interface __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

typedef struct __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __FIVectorView_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl;

interface __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

typedef struct __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl;

interface __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* This,
        __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetectorVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* This,
        __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetectorVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTrackerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTrackerVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTrackerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTrackerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* This,
        __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTrackerVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTrackerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CFaceAnalysis__CFaceTracker_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat;

typedef struct __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl;

interface __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat;

typedef struct __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        __FIIterator_1_Windows__CGraphics__CImaging__CBitmapPixelFormat** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl;

interface __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat;

typedef struct __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        UINT32 index,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl;

interface __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds;

typedef struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize;

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrame __x_ABI_CWindows_CMedia_CIVideoFrame;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

/*
 *
 * Interface Windows.Media.FaceAnalysis.IDetectedFace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.DetectedFace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IDetectedFace[] = L"Windows.Media.FaceAnalysis.IDetectedFace";
typedef struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FaceBox)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds* returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFaceVtbl;

interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_get_FaceBox(This, returnValue) \
    ((This)->lpVtbl->get_FaceBox(This, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIDetectedFace_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.FaceAnalysis.IFaceDetector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.FaceDetector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IFaceDetector[] = L"Windows.Media.FaceAnalysis.IFaceDetector";
typedef struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DetectFacesAsync)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* image,
        __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** returnValue);
    HRESULT (STDMETHODCALLTYPE* DetectFacesWithSearchAreaAsync)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* image,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds searchArea,
        __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** returnValue);
    HRESULT (STDMETHODCALLTYPE* get_MinDetectableFaceSize)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize* returnValue);
    HRESULT (STDMETHODCALLTYPE* put_MinDetectableFaceSize)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize value);
    HRESULT (STDMETHODCALLTYPE* get_MaxDetectableFaceSize)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize* returnValue);
    HRESULT (STDMETHODCALLTYPE* put_MaxDetectableFaceSize)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorVtbl;

interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_DetectFacesAsync(This, image, returnValue) \
    ((This)->lpVtbl->DetectFacesAsync(This, image, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_DetectFacesWithSearchAreaAsync(This, image, searchArea, returnValue) \
    ((This)->lpVtbl->DetectFacesWithSearchAreaAsync(This, image, searchArea, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_get_MinDetectableFaceSize(This, returnValue) \
    ((This)->lpVtbl->get_MinDetectableFaceSize(This, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_put_MinDetectableFaceSize(This, value) \
    ((This)->lpVtbl->put_MinDetectableFaceSize(This, value))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_get_MaxDetectableFaceSize(This, returnValue) \
    ((This)->lpVtbl->get_MaxDetectableFaceSize(This, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_put_MaxDetectableFaceSize(This, value) \
    ((This)->lpVtbl->put_MaxDetectableFaceSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.FaceAnalysis.IFaceDetectorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.FaceDetector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IFaceDetectorStatics[] = L"Windows.Media.FaceAnalysis.IFaceDetectorStatics";
typedef struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This,
        __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceDetector** returnValue);
    HRESULT (STDMETHODCALLTYPE* GetSupportedBitmapPixelFormats)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This,
        __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat** result);
    HRESULT (STDMETHODCALLTYPE* IsBitmapPixelFormatSupported)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat bitmapPixelFormat,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics* This,
        boolean* returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_CreateAsync(This, returnValue) \
    ((This)->lpVtbl->CreateAsync(This, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_GetSupportedBitmapPixelFormats(This, result) \
    ((This)->lpVtbl->GetSupportedBitmapPixelFormats(This, result))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_IsBitmapPixelFormatSupported(This, bitmapPixelFormat, result) \
    ((This)->lpVtbl->IsBitmapPixelFormatSupported(This, bitmapPixelFormat, result))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_get_IsSupported(This, returnValue) \
    ((This)->lpVtbl->get_IsSupported(This, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceDetectorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.FaceAnalysis.IFaceTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.FaceTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IFaceTracker[] = L"Windows.Media.FaceAnalysis.IFaceTracker";
typedef struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ProcessNextFrameAsync)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame* videoFrame,
        __FIAsyncOperation_1___FIVector_1_Windows__CMedia__CFaceAnalysis__CDetectedFace** returnValue);
    HRESULT (STDMETHODCALLTYPE* get_MinDetectableFaceSize)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize* returnValue);
    HRESULT (STDMETHODCALLTYPE* put_MinDetectableFaceSize)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize value);
    HRESULT (STDMETHODCALLTYPE* get_MaxDetectableFaceSize)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize* returnValue);
    HRESULT (STDMETHODCALLTYPE* put_MaxDetectableFaceSize)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerVtbl;

interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_ProcessNextFrameAsync(This, videoFrame, returnValue) \
    ((This)->lpVtbl->ProcessNextFrameAsync(This, videoFrame, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_get_MinDetectableFaceSize(This, returnValue) \
    ((This)->lpVtbl->get_MinDetectableFaceSize(This, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_put_MinDetectableFaceSize(This, value) \
    ((This)->lpVtbl->put_MinDetectableFaceSize(This, value))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_get_MaxDetectableFaceSize(This, returnValue) \
    ((This)->lpVtbl->get_MaxDetectableFaceSize(This, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_put_MaxDetectableFaceSize(This, value) \
    ((This)->lpVtbl->put_MaxDetectableFaceSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTracker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.FaceAnalysis.IFaceTrackerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.FaceAnalysis.FaceTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_FaceAnalysis_IFaceTrackerStatics[] = L"Windows.Media.FaceAnalysis.IFaceTrackerStatics";
typedef struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This,
        __FIAsyncOperation_1_Windows__CMedia__CFaceAnalysis__CFaceTracker** returnValue);
    HRESULT (STDMETHODCALLTYPE* GetSupportedBitmapPixelFormats)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This,
        __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapPixelFormat** result);
    HRESULT (STDMETHODCALLTYPE* IsBitmapPixelFormatSupported)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat bitmapPixelFormat,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics* This,
        boolean* returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_CreateAsync(This, returnValue) \
    ((This)->lpVtbl->CreateAsync(This, returnValue))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_GetSupportedBitmapPixelFormats(This, result) \
    ((This)->lpVtbl->GetSupportedBitmapPixelFormats(This, result))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_IsBitmapPixelFormatSupported(This, bitmapPixelFormat, result) \
    ((This)->lpVtbl->IsBitmapPixelFormatSupported(This, bitmapPixelFormat, result))

#define __x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_get_IsSupported(This, returnValue) \
    ((This)->lpVtbl->get_IsSupported(This, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CFaceAnalysis_CIFaceTrackerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.FaceAnalysis.DetectedFace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.FaceAnalysis.IDetectedFace ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_FaceAnalysis_DetectedFace_DEFINED
#define RUNTIMECLASS_Windows_Media_FaceAnalysis_DetectedFace_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_FaceAnalysis_DetectedFace[] = L"Windows.Media.FaceAnalysis.DetectedFace";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.FaceAnalysis.FaceDetector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.FaceAnalysis.IFaceDetectorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.FaceAnalysis.IFaceDetector ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_FaceAnalysis_FaceDetector_DEFINED
#define RUNTIMECLASS_Windows_Media_FaceAnalysis_FaceDetector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_FaceAnalysis_FaceDetector[] = L"Windows.Media.FaceAnalysis.FaceDetector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.FaceAnalysis.FaceTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.FaceAnalysis.IFaceTrackerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.FaceAnalysis.IFaceTracker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_FaceAnalysis_FaceTracker_DEFINED
#define RUNTIMECLASS_Windows_Media_FaceAnalysis_FaceTracker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_FaceAnalysis_FaceTracker[] = L"Windows.Media.FaceAnalysis.FaceTracker";
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
#endif // __windows2Emedia2Efaceanalysis_p_h__

#endif // __windows2Emedia2Efaceanalysis_h__
