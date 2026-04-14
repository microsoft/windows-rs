
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
#ifndef __windows2Emedia2Eapprecording_h__
#define __windows2Emedia2Eapprecording_h__
#ifndef __windows2Emedia2Eapprecording_p_h__
#define __windows2Emedia2Eapprecording_p_h__


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

#if !defined(WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION)
#define WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Storage.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                interface IAppRecordingManager;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager ABI::Windows::Media::AppRecording::IAppRecordingManager

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                interface IAppRecordingManagerStatics;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics ABI::Windows::Media::AppRecording::IAppRecordingManagerStatics

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                interface IAppRecordingResult;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult ABI::Windows::Media::AppRecording::IAppRecordingResult

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                interface IAppRecordingSaveScreenshotResult;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult ABI::Windows::Media::AppRecording::IAppRecordingSaveScreenshotResult

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                interface IAppRecordingSavedScreenshotInfo;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo ABI::Windows::Media::AppRecording::IAppRecordingSavedScreenshotInfo

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                interface IAppRecordingStatus;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus ABI::Windows::Media::AppRecording::IAppRecordingStatus

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                interface IAppRecordingStatusDetails;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails ABI::Windows::Media::AppRecording::IAppRecordingStatusDetails

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                class AppRecordingResult;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2c72c716-30ea-552c-aaca-51d123234ee3"))
IAsyncOperation<ABI::Windows::Media::AppRecording::AppRecordingResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AppRecording::AppRecordingResult*, ABI::Windows::Media::AppRecording::IAppRecordingResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.AppRecording.AppRecordingResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::AppRecording::AppRecordingResult*> __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_USE */

#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1f6f478f-6cab-58e5-8194-98083c72ddfc"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::AppRecording::AppRecordingResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AppRecording::AppRecordingResult*, ABI::Windows::Media::AppRecording::IAppRecordingResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.AppRecording.AppRecordingResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::AppRecording::AppRecordingResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_USE */

#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                class AppRecordingSaveScreenshotResult;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a048c53e-e624-512b-8e07-ac4e64391b2a"))
IAsyncOperation<ABI::Windows::Media::AppRecording::AppRecordingSaveScreenshotResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AppRecording::AppRecordingSaveScreenshotResult*, ABI::Windows::Media::AppRecording::IAppRecordingSaveScreenshotResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.AppRecording.AppRecordingSaveScreenshotResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::AppRecording::AppRecordingSaveScreenshotResult*> __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_USE */

#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8e2047c3-4cdd-5404-9f68-529d0a35be65"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::AppRecording::AppRecordingSaveScreenshotResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AppRecording::AppRecordingSaveScreenshotResult*, ABI::Windows::Media::AppRecording::IAppRecordingSaveScreenshotResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.AppRecording.AppRecordingSaveScreenshotResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::AppRecording::AppRecordingSaveScreenshotResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_USE */

#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000


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
        namespace Media {
            namespace AppRecording {
                class AppRecordingSavedScreenshotInfo;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE
#define DEF___FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3c407016-1940-5e2b-8830-c54becbbe0da"))
IIterator<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*, ABI::Windows::Media::AppRecording::IAppRecordingSavedScreenshotInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*> __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_t;
#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE */

#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE
#define DEF___FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dd170424-794d-5158-a9af-6824353f91b2"))
IIterable<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*, ABI::Windows::Media::AppRecording::IAppRecordingSavedScreenshotInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*> __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_t;
#define __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE */

#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000


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


#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE
#define DEF___FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("43c83783-b36d-5a8e-b993-e19c823e6c1a"))
IVectorView<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*, ABI::Windows::Media::AppRecording::IAppRecordingSavedScreenshotInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::AppRecording::AppRecordingSavedScreenshotInfo*> __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_t;
#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_USE */

#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

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
        namespace Storage {
            class StorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile ABI::Windows::Storage::IStorageFile

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder ABI::Windows::Storage::IStorageFolder

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                typedef enum AppRecordingSaveScreenshotOption : int AppRecordingSaveScreenshotOption;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                class AppRecordingManager;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                class AppRecordingStatus;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                class AppRecordingStatusDetails;
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.AppRecording.AppRecordingSaveScreenshotOption
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                enum AppRecordingSaveScreenshotOption : int
                {
                    AppRecordingSaveScreenshotOption_None = 0,
                    AppRecordingSaveScreenshotOption_HdrContentVisible = 1,
                };
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingManager
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingManager
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingManager[] = L"Windows.Media.AppRecording.IAppRecordingManager";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                MIDL_INTERFACE("e7e26076-a044-48e2-a512-3094d574c7cc")
                IAppRecordingManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStatus(
                        ABI::Windows::Media::AppRecording::IAppRecordingStatus** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartRecordingToFileAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RecordTimeSpanToFileAsync(
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::Foundation::TimeSpan duration,
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedScreenshotMediaEncodingSubtypes(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveScreenshotToFilesAsync(
                        ABI::Windows::Storage::IStorageFolder* folder,
                        HSTRING filenamePrefix,
                        ABI::Windows::Media::AppRecording::AppRecordingSaveScreenshotOption option,
                        __FIIterable_1_HSTRING* requestedFormats,
                        __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppRecordingManager = __uuidof(IAppRecordingManager);
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingManagerStatics
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingManager
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingManagerStatics[] = L"Windows.Media.AppRecording.IAppRecordingManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                MIDL_INTERFACE("50e709f7-38ce-4bd3-9db2-e72bbe9de11d")
                IAppRecordingManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Media::AppRecording::IAppRecordingManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppRecordingManagerStatics = __uuidof(IAppRecordingManagerStatics);
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingResult
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingResult
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingResult[] = L"Windows.Media.AppRecording.IAppRecordingResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                MIDL_INTERFACE("3a900864-c66d-46f9-b2d9-5bc2dad070d7")
                IAppRecordingResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsFileTruncated(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppRecordingResult = __uuidof(IAppRecordingResult);
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingSaveScreenshotResult
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingSaveScreenshotResult
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingSaveScreenshotResult[] = L"Windows.Media.AppRecording.IAppRecordingSaveScreenshotResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                MIDL_INTERFACE("9c5b8d0a-0abb-4457-aaee-24f9c12ec778")
                IAppRecordingSaveScreenshotResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SavedScreenshotInfos(
                        __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppRecordingSaveScreenshotResult = __uuidof(IAppRecordingSaveScreenshotResult);
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingSavedScreenshotInfo
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingSavedScreenshotInfo[] = L"Windows.Media.AppRecording.IAppRecordingSavedScreenshotInfo";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                MIDL_INTERFACE("9b642d0a-189a-4d00-bf25-e1bb1249d594")
                IAppRecordingSavedScreenshotInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_File(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaEncodingSubtype(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppRecordingSavedScreenshotInfo = __uuidof(IAppRecordingSavedScreenshotInfo);
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingStatus
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingStatus
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingStatus[] = L"Windows.Media.AppRecording.IAppRecordingStatus";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                MIDL_INTERFACE("1d0cc82c-bc18-4b8a-a6ef-127efab3b5d9")
                IAppRecordingStatus : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanRecord(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanRecordTimeSpan(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HistoricalBufferDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Details(
                        ABI::Windows::Media::AppRecording::IAppRecordingStatusDetails** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppRecordingStatus = __uuidof(IAppRecordingStatus);
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingStatusDetails
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingStatusDetails
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingStatusDetails[] = L"Windows.Media.AppRecording.IAppRecordingStatusDetails";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace AppRecording {
                MIDL_INTERFACE("b538a9b0-14ed-4412-ac45-6d672c9c9949")
                IAppRecordingStatusDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsAnyAppBroadcasting(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCaptureResourceUnavailable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsGameStreamInProgress(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsTimeSpanRecordingDisabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsGpuConstrained(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAppInactive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsBlockedForApp(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDisabledByUser(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDisabledBySystem(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppRecordingStatusDetails = __uuidof(IAppRecordingStatusDetails);
            } /* AppRecording */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingManager
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.AppRecording.IAppRecordingManagerStatics interface starting with version 1.0 of the Windows.Media.AppRecording.AppRecordingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingManager_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingManager[] = L"Windows.Media.AppRecording.AppRecordingManager";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingResult
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingResult_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingResult[] = L"Windows.Media.AppRecording.AppRecordingResult";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingSaveScreenshotResult
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingSaveScreenshotResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingSaveScreenshotResult_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingSaveScreenshotResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingSaveScreenshotResult[] = L"Windows.Media.AppRecording.AppRecordingSaveScreenshotResult";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingSavedScreenshotInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingSavedScreenshotInfo_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingSavedScreenshotInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingSavedScreenshotInfo[] = L"Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingStatus
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingStatus ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingStatus_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingStatus[] = L"Windows.Media.AppRecording.AppRecordingStatus";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingStatusDetails
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingStatusDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingStatusDetails_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingStatusDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingStatusDetails[] = L"Windows.Media.AppRecording.AppRecordingStatusDetails";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager;

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics;

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult;

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult;

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo;

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus;

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails;

#endif // ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult;

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult_INTERFACE_DEFINED__
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingResult_INTERFACE_DEFINED__
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult;

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

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

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo;

typedef struct __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl;

interface __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo;

typedef struct __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        __FIIterator_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl;

interface __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

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

#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo;

typedef struct __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl;

interface __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CAppRecording_CAppRecordingSaveScreenshotOption __x_ABI_CWindows_CMedia_CAppRecording_CAppRecordingSaveScreenshotOption;

/*
 *
 * Struct Windows.Media.AppRecording.AppRecordingSaveScreenshotOption
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CAppRecording_CAppRecordingSaveScreenshotOption
{
    AppRecordingSaveScreenshotOption_None = 0,
    AppRecordingSaveScreenshotOption_HdrContentVisible = 1,
};
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingManager
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingManager
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingManager[] = L"Windows.Media.AppRecording.IAppRecordingManager";
typedef struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStatus)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus** result);
    HRESULT (STDMETHODCALLTYPE* StartRecordingToFileAsync)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult** operation);
    HRESULT (STDMETHODCALLTYPE* RecordTimeSpanToFileAsync)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingResult** operation);
    HRESULT (STDMETHODCALLTYPE* get_SupportedScreenshotMediaEncodingSubtypes)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* SaveScreenshotToFilesAsync)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* folder,
        HSTRING filenamePrefix,
        enum __x_ABI_CWindows_CMedia_CAppRecording_CAppRecordingSaveScreenshotOption option,
        __FIIterable_1_HSTRING* requestedFormats,
        __FIAsyncOperation_1_Windows__CMedia__CAppRecording__CAppRecordingSaveScreenshotResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerVtbl;

interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_GetStatus(This, result) \
    ((This)->lpVtbl->GetStatus(This, result))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_StartRecordingToFileAsync(This, file, operation) \
    ((This)->lpVtbl->StartRecordingToFileAsync(This, file, operation))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_RecordTimeSpanToFileAsync(This, startTime, duration, file, operation) \
    ((This)->lpVtbl->RecordTimeSpanToFileAsync(This, startTime, duration, file, operation))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_get_SupportedScreenshotMediaEncodingSubtypes(This, value) \
    ((This)->lpVtbl->get_SupportedScreenshotMediaEncodingSubtypes(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_SaveScreenshotToFilesAsync(This, folder, filenamePrefix, option, requestedFormats, operation) \
    ((This)->lpVtbl->SaveScreenshotToFilesAsync(This, folder, filenamePrefix, option, requestedFormats, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingManagerStatics
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingManager
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingManagerStatics[] = L"Windows.Media.AppRecording.IAppRecordingManagerStatics";
typedef struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics* This,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingResult
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingResult
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingResult[] = L"Windows.Media.AppRecording.IAppRecordingResult";
typedef struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_IsFileTruncated)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResultVtbl;

interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_get_IsFileTruncated(This, value) \
    ((This)->lpVtbl->get_IsFileTruncated(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingSaveScreenshotResult
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingSaveScreenshotResult
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingSaveScreenshotResult[] = L"Windows.Media.AppRecording.IAppRecordingSaveScreenshotResult";
typedef struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_SavedScreenshotInfos)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult* This,
        __FIVectorView_1_Windows__CMedia__CAppRecording__CAppRecordingSavedScreenshotInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResultVtbl;

interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_get_SavedScreenshotInfos(This, value) \
    ((This)->lpVtbl->get_SavedScreenshotInfos(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSaveScreenshotResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingSavedScreenshotInfo
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingSavedScreenshotInfo[] = L"Windows.Media.AppRecording.IAppRecordingSavedScreenshotInfo";
typedef struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_File)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
    HRESULT (STDMETHODCALLTYPE* get_MediaEncodingSubtype)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfoVtbl;

interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_get_File(This, value) \
    ((This)->lpVtbl->get_File(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_get_MediaEncodingSubtype(This, value) \
    ((This)->lpVtbl->get_MediaEncodingSubtype(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingSavedScreenshotInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingStatus
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingStatus
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingStatus[] = L"Windows.Media.AppRecording.IAppRecordingStatus";
typedef struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanRecord)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CanRecordTimeSpan)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HistoricalBufferDuration)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Details)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus* This,
        __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusVtbl;

interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_get_CanRecord(This, value) \
    ((This)->lpVtbl->get_CanRecord(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_get_CanRecordTimeSpan(This, value) \
    ((This)->lpVtbl->get_CanRecordTimeSpan(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_get_HistoricalBufferDuration(This, value) \
    ((This)->lpVtbl->get_HistoricalBufferDuration(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_get_Details(This, value) \
    ((This)->lpVtbl->get_Details(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.AppRecording.IAppRecordingStatusDetails
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AppRecording.AppRecordingStatusDetails
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_AppRecording_IAppRecordingStatusDetails[] = L"Windows.Media.AppRecording.IAppRecordingStatusDetails";
typedef struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAnyAppBroadcasting)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCaptureResourceUnavailable)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsGameStreamInProgress)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsTimeSpanRecordingDisabled)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsGpuConstrained)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAppInactive)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsBlockedForApp)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDisabledByUser)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDisabledBySystem)(__x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetailsVtbl;

interface __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsAnyAppBroadcasting(This, value) \
    ((This)->lpVtbl->get_IsAnyAppBroadcasting(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsCaptureResourceUnavailable(This, value) \
    ((This)->lpVtbl->get_IsCaptureResourceUnavailable(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsGameStreamInProgress(This, value) \
    ((This)->lpVtbl->get_IsGameStreamInProgress(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsTimeSpanRecordingDisabled(This, value) \
    ((This)->lpVtbl->get_IsTimeSpanRecordingDisabled(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsGpuConstrained(This, value) \
    ((This)->lpVtbl->get_IsGpuConstrained(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsAppInactive(This, value) \
    ((This)->lpVtbl->get_IsAppInactive(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsBlockedForApp(This, value) \
    ((This)->lpVtbl->get_IsBlockedForApp(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsDisabledByUser(This, value) \
    ((This)->lpVtbl->get_IsDisabledByUser(This, value))

#define __x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_get_IsDisabledBySystem(This, value) \
    ((This)->lpVtbl->get_IsDisabledBySystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAppRecording_CIAppRecordingStatusDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingManager
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.AppRecording.IAppRecordingManagerStatics interface starting with version 1.0 of the Windows.Media.AppRecording.AppRecordingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingManager_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingManager[] = L"Windows.Media.AppRecording.AppRecordingManager";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingResult
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingResult_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingResult[] = L"Windows.Media.AppRecording.AppRecordingResult";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingSaveScreenshotResult
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingSaveScreenshotResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingSaveScreenshotResult_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingSaveScreenshotResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingSaveScreenshotResult[] = L"Windows.Media.AppRecording.AppRecordingSaveScreenshotResult";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingSavedScreenshotInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingSavedScreenshotInfo_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingSavedScreenshotInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingSavedScreenshotInfo[] = L"Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingStatus
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingStatus ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingStatus_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingStatus[] = L"Windows.Media.AppRecording.AppRecordingStatus";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AppRecording.AppRecordingStatusDetails
 *
 * Introduced to Windows.Media.AppRecording.AppRecordingContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.AppRecording.IAppRecordingStatusDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingStatusDetails_DEFINED
#define RUNTIMECLASS_Windows_Media_AppRecording_AppRecordingStatusDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AppRecording_AppRecordingStatusDetails[] = L"Windows.Media.AppRecording.AppRecordingStatusDetails";
#endif
#endif // WINDOWS_MEDIA_APPRECORDING_APPRECORDINGCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Eapprecording_p_h__

#endif // __windows2Emedia2Eapprecording_h__
