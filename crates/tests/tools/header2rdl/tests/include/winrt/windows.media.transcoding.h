
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
#ifndef __windows2Emedia2Etranscoding_h__
#define __windows2Emedia2Etranscoding_h__
#ifndef __windows2Emedia2Etranscoding_p_h__
#define __windows2Emedia2Etranscoding_p_h__


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
#include "Windows.Media.Core.h"
#include "Windows.Media.MediaProperties.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                interface IMediaTranscoder;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder ABI::Windows::Media::Transcoding::IMediaTranscoder

#endif // ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                interface IMediaTranscoder2;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2 ABI::Windows::Media::Transcoding::IMediaTranscoder2

#endif // ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                interface IPrepareTranscodeResult;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult ABI::Windows::Media::Transcoding::IPrepareTranscodeResult

#endif // ____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncActionWithProgressCompletedHandler_1_double_USE
#define DEF___FIAsyncActionWithProgressCompletedHandler_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("94d64ac6-4491-53ef-8be8-36481f3ff1e8"))
IAsyncActionWithProgressCompletedHandler<double> : IAsyncActionWithProgressCompletedHandler_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncActionWithProgressCompletedHandler`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncActionWithProgressCompletedHandler<double> __FIAsyncActionWithProgressCompletedHandler_1_double_t;
#define __FIAsyncActionWithProgressCompletedHandler_1_double ABI::Windows::Foundation::__FIAsyncActionWithProgressCompletedHandler_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncActionWithProgressCompletedHandler_1_double_USE */



#ifndef DEF___FIAsyncActionWithProgress_1_double_USE
#define DEF___FIAsyncActionWithProgress_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4f1430a6-a825-56ca-b047-1a9bad52ba67"))
IAsyncActionWithProgress<double> : IAsyncActionWithProgress_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncActionWithProgress`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncActionWithProgress<double> __FIAsyncActionWithProgress_1_double_t;
#define __FIAsyncActionWithProgress_1_double ABI::Windows::Foundation::__FIAsyncActionWithProgress_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncActionWithProgress_1_double_USE */



#ifndef DEF___FIAsyncActionProgressHandler_1_double_USE
#define DEF___FIAsyncActionProgressHandler_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44825c7c-0da9-5691-b2b4-914f231eeced"))
IAsyncActionProgressHandler<double> : IAsyncActionProgressHandler_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncActionProgressHandler`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncActionProgressHandler<double> __FIAsyncActionProgressHandler_1_double_t;
#define __FIAsyncActionProgressHandler_1_double ABI::Windows::Foundation::__FIAsyncActionProgressHandler_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncActionProgressHandler_1_double_USE */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                class PrepareTranscodeResult;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f5f07c13-3047-5bab-8eb7-6e5d7d14afae"))
IAsyncOperation<ABI::Windows::Media::Transcoding::PrepareTranscodeResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Transcoding::PrepareTranscodeResult*, ABI::Windows::Media::Transcoding::IPrepareTranscodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Transcoding.PrepareTranscodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Transcoding::PrepareTranscodeResult*> __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a44d7d07-8f74-52ee-9f02-c2b244b4ff2a"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Transcoding::PrepareTranscodeResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Transcoding::PrepareTranscodeResult*, ABI::Windows::Media::Transcoding::IPrepareTranscodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Transcoding.PrepareTranscodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Transcoding::PrepareTranscodeResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IMediaSource;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIMediaSource ABI::Windows::Media::Core::IMediaSource

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class MediaEncodingProfile;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfile;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile ABI::Windows::Media::MediaProperties::IMediaEncodingProfile

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__

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
        namespace Media {
            namespace Transcoding {
                typedef enum MediaVideoProcessingAlgorithm : int MediaVideoProcessingAlgorithm;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                typedef enum TranscodeFailureReason : int TranscodeFailureReason;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Transcoding.MediaVideoProcessingAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                enum MediaVideoProcessingAlgorithm : int
                {
                    MediaVideoProcessingAlgorithm_Default = 0,
                    MediaVideoProcessingAlgorithm_MrfCrf444 = 1,
                };
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Transcoding.TranscodeFailureReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                enum TranscodeFailureReason : int
                {
                    TranscodeFailureReason_None = 0,
                    TranscodeFailureReason_Unknown = 1,
                    TranscodeFailureReason_InvalidProfile = 2,
                    TranscodeFailureReason_CodecNotFound = 3,
                };
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Transcoding.IMediaTranscoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Transcoding.MediaTranscoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Transcoding_IMediaTranscoder[] = L"Windows.Media.Transcoding.IMediaTranscoder";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                MIDL_INTERFACE("190c99d2-a0aa-4d34-86bc-eed1b12c2f5b")
                IMediaTranscoder : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_TrimStartTime(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrimStartTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TrimStopTime(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrimStopTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AlwaysReencode(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlwaysReencode(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HardwareAccelerationEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareAccelerationEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddAudioEffect(
                        HSTRING activatableClassId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddAudioEffectWithSettings(
                        HSTRING activatableClassId,
                        boolean effectRequired,
                        ABI::Windows::Foundation::Collections::IPropertySet* configuration
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddVideoEffect(
                        HSTRING activatableClassId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddVideoEffectWithSettings(
                        HSTRING activatableClassId,
                        boolean effectRequired,
                        ABI::Windows::Foundation::Collections::IPropertySet* configuration
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearEffects(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PrepareFileTranscodeAsync(
                        ABI::Windows::Storage::IStorageFile* source,
                        ABI::Windows::Storage::IStorageFile* destination,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile* profile,
                        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PrepareStreamTranscodeAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* source,
                        ABI::Windows::Storage::Streams::IRandomAccessStream* destination,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile* profile,
                        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaTranscoder = __uuidof(IMediaTranscoder);
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder;
#endif /* !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Transcoding.IMediaTranscoder2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Transcoding.MediaTranscoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Transcoding_IMediaTranscoder2[] = L"Windows.Media.Transcoding.IMediaTranscoder2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                MIDL_INTERFACE("40531d74-35e0-4f04-8574-ca8bc4e5a082")
                IMediaTranscoder2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE PrepareMediaStreamSourceTranscodeAsync(
                        ABI::Windows::Media::Core::IMediaSource* source,
                        ABI::Windows::Storage::Streams::IRandomAccessStream* destination,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile* profile,
                        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_VideoProcessingAlgorithm(
                        ABI::Windows::Media::Transcoding::MediaVideoProcessingAlgorithm value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideoProcessingAlgorithm(
                        ABI::Windows::Media::Transcoding::MediaVideoProcessingAlgorithm* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaTranscoder2 = __uuidof(IMediaTranscoder2);
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Transcoding.IPrepareTranscodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Transcoding.PrepareTranscodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Transcoding_IPrepareTranscodeResult[] = L"Windows.Media.Transcoding.IPrepareTranscodeResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                MIDL_INTERFACE("05f25dce-994f-4a34-9d68-97ccce1730d6")
                IPrepareTranscodeResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanTranscode(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FailureReason(
                        ABI::Windows::Media::Transcoding::TranscodeFailureReason* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TranscodeAsync(
                        __FIAsyncActionWithProgress_1_double** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrepareTranscodeResult = __uuidof(IPrepareTranscodeResult);
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Transcoding.MediaTranscoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Transcoding.IMediaTranscoder ** Default Interface **
 *    Windows.Media.Transcoding.IMediaTranscoder2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Transcoding_MediaTranscoder_DEFINED
#define RUNTIMECLASS_Windows_Media_Transcoding_MediaTranscoder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Transcoding_MediaTranscoder[] = L"Windows.Media.Transcoding.MediaTranscoder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Transcoding.PrepareTranscodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Transcoding.IPrepareTranscodeResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Transcoding_PrepareTranscodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Transcoding_PrepareTranscodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Transcoding_PrepareTranscodeResult[] = L"Windows.Media.Transcoding.PrepareTranscodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder;

#endif // ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2 __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2;

#endif // ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult;

#endif // ____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncActionProgressHandler_1_double __FIAsyncActionProgressHandler_1_double;

typedef interface __FIAsyncActionWithProgress_1_double __FIAsyncActionWithProgress_1_double;

#if !defined(____FIAsyncActionWithProgressCompletedHandler_1_double_INTERFACE_DEFINED__)
#define ____FIAsyncActionWithProgressCompletedHandler_1_double_INTERFACE_DEFINED__

typedef interface __FIAsyncActionWithProgressCompletedHandler_1_double __FIAsyncActionWithProgressCompletedHandler_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncActionWithProgressCompletedHandler_1_double;

typedef struct __FIAsyncActionWithProgressCompletedHandler_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncActionWithProgressCompletedHandler_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncActionWithProgressCompletedHandler_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncActionWithProgressCompletedHandler_1_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncActionWithProgressCompletedHandler_1_double* This,
        __FIAsyncActionWithProgress_1_double* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncActionWithProgressCompletedHandler_1_doubleVtbl;

interface __FIAsyncActionWithProgressCompletedHandler_1_double
{
    CONST_VTBL struct __FIAsyncActionWithProgressCompletedHandler_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncActionWithProgressCompletedHandler_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncActionWithProgressCompletedHandler_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncActionWithProgressCompletedHandler_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncActionWithProgressCompletedHandler_1_double_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncActionWithProgressCompletedHandler_1_double_INTERFACE_DEFINED__

#if !defined(____FIAsyncActionWithProgress_1_double_INTERFACE_DEFINED__)
#define ____FIAsyncActionWithProgress_1_double_INTERFACE_DEFINED__

typedef interface __FIAsyncActionWithProgress_1_double __FIAsyncActionWithProgress_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncActionWithProgress_1_double;

typedef struct __FIAsyncActionWithProgress_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncActionWithProgress_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncActionWithProgress_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncActionWithProgress_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncActionWithProgress_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncActionWithProgress_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncActionWithProgress_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncActionWithProgress_1_double* This,
        __FIAsyncActionProgressHandler_1_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncActionWithProgress_1_double* This,
        __FIAsyncActionProgressHandler_1_double** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncActionWithProgress_1_double* This,
        __FIAsyncActionWithProgressCompletedHandler_1_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncActionWithProgress_1_double* This,
        __FIAsyncActionWithProgressCompletedHandler_1_double** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncActionWithProgress_1_double* This);

    END_INTERFACE
} __FIAsyncActionWithProgress_1_doubleVtbl;

interface __FIAsyncActionWithProgress_1_double
{
    CONST_VTBL struct __FIAsyncActionWithProgress_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncActionWithProgress_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncActionWithProgress_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncActionWithProgress_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncActionWithProgress_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncActionWithProgress_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncActionWithProgress_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncActionWithProgress_1_double_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncActionWithProgress_1_double_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncActionWithProgress_1_double_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncActionWithProgress_1_double_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncActionWithProgress_1_double_GetResults(This) \
    ((This)->lpVtbl->GetResults(This))

#endif /* COBJMACROS */

#endif // ____FIAsyncActionWithProgress_1_double_INTERFACE_DEFINED__

#if !defined(____FIAsyncActionProgressHandler_1_double_INTERFACE_DEFINED__)
#define ____FIAsyncActionProgressHandler_1_double_INTERFACE_DEFINED__

typedef interface __FIAsyncActionProgressHandler_1_double __FIAsyncActionProgressHandler_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncActionProgressHandler_1_double;

typedef struct __FIAsyncActionProgressHandler_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncActionProgressHandler_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncActionProgressHandler_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncActionProgressHandler_1_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncActionProgressHandler_1_double* This,
        __FIAsyncActionWithProgress_1_double* asyncInfo,
        DOUBLE progressInfo);

    END_INTERFACE
} __FIAsyncActionProgressHandler_1_doubleVtbl;

interface __FIAsyncActionProgressHandler_1_double
{
    CONST_VTBL struct __FIAsyncActionProgressHandler_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncActionProgressHandler_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncActionProgressHandler_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncActionProgressHandler_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncActionProgressHandler_1_double_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncActionProgressHandler_1_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaSource __x_ABI_CWindows_CMedia_CCore_CIMediaSource;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm;

typedef enum __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason;

/*
 *
 * Struct Windows.Media.Transcoding.MediaVideoProcessingAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm
{
    MediaVideoProcessingAlgorithm_Default = 0,
    MediaVideoProcessingAlgorithm_MrfCrf444 = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Transcoding.TranscodeFailureReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason
{
    TranscodeFailureReason_None = 0,
    TranscodeFailureReason_Unknown = 1,
    TranscodeFailureReason_InvalidProfile = 2,
    TranscodeFailureReason_CodecNotFound = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Transcoding.IMediaTranscoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Transcoding.MediaTranscoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Transcoding_IMediaTranscoder[] = L"Windows.Media.Transcoding.IMediaTranscoder";
typedef struct __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_TrimStartTime)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_TrimStartTime)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_TrimStopTime)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_TrimStopTime)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_AlwaysReencode)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AlwaysReencode)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_HardwareAccelerationEnabled)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareAccelerationEnabled)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* AddAudioEffect)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        HSTRING activatableClassId);
    HRESULT (STDMETHODCALLTYPE* AddAudioEffectWithSettings)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        HSTRING activatableClassId,
        boolean effectRequired,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);
    HRESULT (STDMETHODCALLTYPE* AddVideoEffect)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        HSTRING activatableClassId);
    HRESULT (STDMETHODCALLTYPE* AddVideoEffectWithSettings)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        HSTRING activatableClassId,
        boolean effectRequired,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);
    HRESULT (STDMETHODCALLTYPE* ClearEffects)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This);
    HRESULT (STDMETHODCALLTYPE* PrepareFileTranscodeAsync)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* source,
        __x_ABI_CWindows_CStorage_CIStorageFile* destination,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* profile,
        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult** operation);
    HRESULT (STDMETHODCALLTYPE* PrepareStreamTranscodeAsync)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* source,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* destination,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* profile,
        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoderVtbl;

interface __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_put_TrimStartTime(This, value) \
    ((This)->lpVtbl->put_TrimStartTime(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_get_TrimStartTime(This, value) \
    ((This)->lpVtbl->get_TrimStartTime(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_put_TrimStopTime(This, value) \
    ((This)->lpVtbl->put_TrimStopTime(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_get_TrimStopTime(This, value) \
    ((This)->lpVtbl->get_TrimStopTime(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_put_AlwaysReencode(This, value) \
    ((This)->lpVtbl->put_AlwaysReencode(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_get_AlwaysReencode(This, value) \
    ((This)->lpVtbl->get_AlwaysReencode(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_put_HardwareAccelerationEnabled(This, value) \
    ((This)->lpVtbl->put_HardwareAccelerationEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_get_HardwareAccelerationEnabled(This, value) \
    ((This)->lpVtbl->get_HardwareAccelerationEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_AddAudioEffect(This, activatableClassId) \
    ((This)->lpVtbl->AddAudioEffect(This, activatableClassId))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_AddAudioEffectWithSettings(This, activatableClassId, effectRequired, configuration) \
    ((This)->lpVtbl->AddAudioEffectWithSettings(This, activatableClassId, effectRequired, configuration))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_AddVideoEffect(This, activatableClassId) \
    ((This)->lpVtbl->AddVideoEffect(This, activatableClassId))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_AddVideoEffectWithSettings(This, activatableClassId, effectRequired, configuration) \
    ((This)->lpVtbl->AddVideoEffectWithSettings(This, activatableClassId, effectRequired, configuration))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_ClearEffects(This) \
    ((This)->lpVtbl->ClearEffects(This))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_PrepareFileTranscodeAsync(This, source, destination, profile, operation) \
    ((This)->lpVtbl->PrepareFileTranscodeAsync(This, source, destination, profile, operation))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_PrepareStreamTranscodeAsync(This, source, destination, profile, operation) \
    ((This)->lpVtbl->PrepareStreamTranscodeAsync(This, source, destination, profile, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder;
#endif /* !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Transcoding.IMediaTranscoder2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Transcoding.MediaTranscoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Transcoding_IMediaTranscoder2[] = L"Windows.Media.Transcoding.IMediaTranscoder2";
typedef struct __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* PrepareMediaStreamSourceTranscodeAsync)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaSource* source,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* destination,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* profile,
        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CPrepareTranscodeResult** operation);
    HRESULT (STDMETHODCALLTYPE* put_VideoProcessingAlgorithm)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This,
        enum __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm value);
    HRESULT (STDMETHODCALLTYPE* get_VideoProcessingAlgorithm)(__x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2* This,
        enum __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2Vtbl;

interface __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_PrepareMediaStreamSourceTranscodeAsync(This, source, destination, profile, operation) \
    ((This)->lpVtbl->PrepareMediaStreamSourceTranscodeAsync(This, source, destination, profile, operation))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_put_VideoProcessingAlgorithm(This, value) \
    ((This)->lpVtbl->put_VideoProcessingAlgorithm(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_get_VideoProcessingAlgorithm(This, value) \
    ((This)->lpVtbl->get_VideoProcessingAlgorithm(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIMediaTranscoder2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Transcoding.IPrepareTranscodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Transcoding.PrepareTranscodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Transcoding_IPrepareTranscodeResult[] = L"Windows.Media.Transcoding.IPrepareTranscodeResult";
typedef struct __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanTranscode)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_FailureReason)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This,
        enum __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason* value);
    HRESULT (STDMETHODCALLTYPE* TranscodeAsync)(__x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult* This,
        __FIAsyncActionWithProgress_1_double** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResultVtbl;

interface __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_get_CanTranscode(This, value) \
    ((This)->lpVtbl->get_CanTranscode(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_get_FailureReason(This, value) \
    ((This)->lpVtbl->get_FailureReason(This, value))

#define __x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_TranscodeAsync(This, operation) \
    ((This)->lpVtbl->TranscodeAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CTranscoding_CIPrepareTranscodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Transcoding.MediaTranscoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Transcoding.IMediaTranscoder ** Default Interface **
 *    Windows.Media.Transcoding.IMediaTranscoder2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Transcoding_MediaTranscoder_DEFINED
#define RUNTIMECLASS_Windows_Media_Transcoding_MediaTranscoder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Transcoding_MediaTranscoder[] = L"Windows.Media.Transcoding.MediaTranscoder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Transcoding.PrepareTranscodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Transcoding.IPrepareTranscodeResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Transcoding_PrepareTranscodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Transcoding_PrepareTranscodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Transcoding_PrepareTranscodeResult[] = L"Windows.Media.Transcoding.PrepareTranscodeResult";
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
#endif // __windows2Emedia2Etranscoding_p_h__

#endif // __windows2Emedia2Etranscoding_h__
