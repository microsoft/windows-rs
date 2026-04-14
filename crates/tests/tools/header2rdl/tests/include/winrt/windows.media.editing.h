
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
#ifndef __windows2Emedia2Eediting_h__
#define __windows2Emedia2Eediting_h__
#ifndef __windows2Emedia2Eediting_p_h__
#define __windows2Emedia2Eediting_p_h__


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
#include "Windows.Graphics.DirectX.Direct3D11.h"
#include "Windows.Media.Core.h"
#include "Windows.Media.Effects.h"
#include "Windows.Media.MediaProperties.h"
#include "Windows.Media.Transcoding.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IBackgroundAudioTrack;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack ABI::Windows::Media::Editing::IBackgroundAudioTrack

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IBackgroundAudioTrackStatics;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics ABI::Windows::Media::Editing::IBackgroundAudioTrackStatics

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IEmbeddedAudioTrack;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack ABI::Windows::Media::Editing::IEmbeddedAudioTrack

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaClip;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip ABI::Windows::Media::Editing::IMediaClip

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaClipStatics;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics ABI::Windows::Media::Editing::IMediaClipStatics

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaClipStatics2;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2 ABI::Windows::Media::Editing::IMediaClipStatics2

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaComposition;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition ABI::Windows::Media::Editing::IMediaComposition

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaComposition2;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2 ABI::Windows::Media::Editing::IMediaComposition2

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaCompositionStatics;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics ABI::Windows::Media::Editing::IMediaCompositionStatics

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaOverlay;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay ABI::Windows::Media::Editing::IMediaOverlay

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaOverlayFactory;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory ABI::Windows::Media::Editing::IMediaOverlayFactory

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaOverlayLayer;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer ABI::Windows::Media::Editing::IMediaOverlayLayer

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaOverlayLayerFactory;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory ABI::Windows::Media::Editing::IMediaOverlayLayerFactory

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class ImageStream;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CImaging__CImageStream_USE
#define DEF___FIIterator_1_Windows__CGraphics__CImaging__CImageStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4a10752d-6b1a-5fec-a59c-70389bf162a2"))
IIterator<ABI::Windows::Graphics::Imaging::ImageStream*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::ImageStream*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Imaging.ImageStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Imaging::ImageStream*> __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_t;
#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CImaging__CImageStream_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CImaging__CImageStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CImaging__CImageStream_USE
#define DEF___FIIterable_1_Windows__CGraphics__CImaging__CImageStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("034ea0c4-c20e-5c0c-ba31-64212f28e650"))
IIterable<ABI::Windows::Graphics::Imaging::ImageStream*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::ImageStream*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Imaging.ImageStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Imaging::ImageStream*> __FIIterable_1_Windows__CGraphics__CImaging__CImageStream_t;
#define __FIIterable_1_Windows__CGraphics__CImaging__CImageStream ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CImaging__CImageStream_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CImaging__CImageStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ab10f3e5-2a3e-5f81-b5e8-8ddddc23cca2"))
IVectorView<ABI::Windows::Graphics::Imaging::ImageStream*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::ImageStream*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Imaging.ImageStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Imaging::ImageStream*> __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_t;
#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a58d867e-beef-5f17-b7cf-e4c87be22ee4"))
IAsyncOperation<__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Imaging.ImageStream>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream*> __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6683d49c-9fd5-5b08-899f-e2d7dc5cf9c4"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Imaging.ImageStream>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("684165be-0011-56d6-bebf-430016d51b7a"))
IAsyncOperation<ABI::Windows::Graphics::Imaging::ImageStream*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::ImageStream*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Imaging.ImageStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Imaging::ImageStream*> __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("29bb8288-4462-516e-a675-8c9235c42994"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::ImageStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::ImageStream*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Imaging.ImageStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::ImageStream*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                class BackgroundAudioTrack;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("493dc898-6076-55f5-ab84-ccf973ac0397"))
IAsyncOperation<ABI::Windows::Media::Editing::BackgroundAudioTrack*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::BackgroundAudioTrack*, ABI::Windows::Media::Editing::IBackgroundAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Editing.BackgroundAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Editing::BackgroundAudioTrack*> __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t;
#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b8830bc7-188b-5c25-a3bb-959052bcb740"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Editing::BackgroundAudioTrack*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::BackgroundAudioTrack*, ABI::Windows::Media::Editing::IBackgroundAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Editing.BackgroundAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Editing::BackgroundAudioTrack*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                class MediaClip;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b5e699dd-b6f1-51c0-b752-e02fa0068d79"))
IAsyncOperation<ABI::Windows::Media::Editing::MediaClip*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaClip*, ABI::Windows::Media::Editing::IMediaClip*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Editing.MediaClip>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Editing::MediaClip*> __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_t;
#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("60cde3bc-119f-50a8-9ccb-9ea57ea96bfd"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Editing::MediaClip*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaClip*, ABI::Windows::Media::Editing::IMediaClip*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Editing.MediaClip>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Editing::MediaClip*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                class MediaComposition;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e170e898-d11f-5054-ab13-1080a4807636"))
IAsyncOperation<ABI::Windows::Media::Editing::MediaComposition*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaComposition*, ABI::Windows::Media::Editing::IMediaComposition*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Editing.MediaComposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Editing::MediaComposition*> __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_t;
#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("edd253f8-4ebd-56e5-9592-3c09375ebdc4"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Editing::MediaComposition*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaComposition*, ABI::Windows::Media::Editing::IMediaComposition*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Editing.MediaComposition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Editing::MediaComposition*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                typedef enum TranscodeFailureReason : int TranscodeFailureReason;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("080f1890-4fca-5165-a989-4b07da8e0b53"))
IAsyncOperationWithProgressCompletedHandler<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double> : IAsyncOperationWithProgressCompletedHandler_impl<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Media.Transcoding.TranscodeFailureReason, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("272eec20-4b64-5d53-a644-f9917b3d19d8"))
IAsyncOperationWithProgress<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double> : IAsyncOperationWithProgress_impl<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Media.Transcoding.TranscodeFailureReason, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double> __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_t;
#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("009c6245-0e59-53b0-9fd2-d250e45a00a3"))
IAsyncOperationProgressHandler<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double> : IAsyncOperationProgressHandler_impl<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Media.Transcoding.TranscodeFailureReason, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason, double> __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CTimeSpan_USE
#define DEF___FIIterator_1_Windows__CFoundation__CTimeSpan_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("67e9eadb-324b-5661-a405-ded8445b1eea"))
IIterator<struct ABI::Windows::Foundation::TimeSpan> : IIterator_impl<struct ABI::Windows::Foundation::TimeSpan>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.TimeSpan>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Foundation::TimeSpan> __FIIterator_1_Windows__CFoundation__CTimeSpan_t;
#define __FIIterator_1_Windows__CFoundation__CTimeSpan ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CTimeSpan_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CTimeSpan_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CTimeSpan_USE
#define DEF___FIIterable_1_Windows__CFoundation__CTimeSpan_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9f78726-829a-5f67-8d19-95ef154b7742"))
IIterable<struct ABI::Windows::Foundation::TimeSpan> : IIterable_impl<struct ABI::Windows::Foundation::TimeSpan>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.TimeSpan>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Foundation::TimeSpan> __FIIterable_1_Windows__CFoundation__CTimeSpan_t;
#define __FIIterable_1_Windows__CFoundation__CTimeSpan ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CTimeSpan_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CTimeSpan_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#define DEF___FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("18b0f692-8635-577a-9efa-cb904770d6e5"))
IIterator<ABI::Windows::Media::Editing::BackgroundAudioTrack*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::BackgroundAudioTrack*, ABI::Windows::Media::Editing::IBackgroundAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Editing.BackgroundAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Editing::BackgroundAudioTrack*> __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t;
#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#define DEF___FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3fc05ba2-30f6-5219-9047-1197ffae8dba"))
IIterable<ABI::Windows::Media::Editing::BackgroundAudioTrack*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::BackgroundAudioTrack*, ABI::Windows::Media::Editing::IBackgroundAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Editing.BackgroundAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Editing::BackgroundAudioTrack*> __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t;
#define __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                class EmbeddedAudioTrack;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE
#define DEF___FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c9fa7f51-b52c-578e-99f9-012cd5641247"))
IIterator<ABI::Windows::Media::Editing::EmbeddedAudioTrack*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::EmbeddedAudioTrack*, ABI::Windows::Media::Editing::IEmbeddedAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Editing.EmbeddedAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Editing::EmbeddedAudioTrack*> __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_t;
#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE
#define DEF___FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("32fabed0-e1e6-578e-86e5-b4e6abeb22d6"))
IIterable<ABI::Windows::Media::Editing::EmbeddedAudioTrack*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::EmbeddedAudioTrack*, ABI::Windows::Media::Editing::IEmbeddedAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Editing.EmbeddedAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Editing::EmbeddedAudioTrack*> __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_t;
#define __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaClip_USE
#define DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaClip_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2dfd20b2-06d8-577d-ab06-771e0414fc00"))
IIterator<ABI::Windows::Media::Editing::MediaClip*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaClip*, ABI::Windows::Media::Editing::IMediaClip*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Editing.MediaClip>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Editing::MediaClip*> __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_t;
#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEditing__CMediaClip_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaClip_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaClip_USE
#define DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaClip_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ff2e9b5c-26d0-575d-a3eb-7d938bd16f17"))
IIterable<ABI::Windows::Media::Editing::MediaClip*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaClip*, ABI::Windows::Media::Editing::IMediaClip*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Editing.MediaClip>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Editing::MediaClip*> __FIIterable_1_Windows__CMedia__CEditing__CMediaClip_t;
#define __FIIterable_1_Windows__CMedia__CEditing__CMediaClip ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEditing__CMediaClip_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaClip_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                class MediaOverlay;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_USE
#define DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("351f6f24-4a92-56d6-a187-faeae748e0c7"))
IIterator<ABI::Windows::Media::Editing::MediaOverlay*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaOverlay*, ABI::Windows::Media::Editing::IMediaOverlay*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Editing.MediaOverlay>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Editing::MediaOverlay*> __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_t;
#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_USE
#define DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("efcda247-a1fb-51dc-a776-e3e06695fb36"))
IIterable<ABI::Windows::Media::Editing::MediaOverlay*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaOverlay*, ABI::Windows::Media::Editing::IMediaOverlay*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Editing.MediaOverlay>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Editing::MediaOverlay*> __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_t;
#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                class MediaOverlayLayer;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE
#define DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ca401ed8-63d2-525a-80bb-e494900c4ce6"))
IIterator<ABI::Windows::Media::Editing::MediaOverlayLayer*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaOverlayLayer*, ABI::Windows::Media::Editing::IMediaOverlayLayer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Editing.MediaOverlayLayer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Editing::MediaOverlayLayer*> __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_t;
#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE
#define DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7d2312d0-f3a2-5091-8a5e-41832e632c08"))
IIterable<ABI::Windows::Media::Editing::MediaOverlayLayer*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaOverlayLayer*, ABI::Windows::Media::Editing::IMediaOverlayLayer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Editing.MediaOverlayLayer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Editing::MediaOverlayLayer*> __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_t;
#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioEffectDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition ABI::Windows::Media::Effects::IAudioEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#define DEF___FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ec0f39fc-6959-5423-9e1a-f7cb8e845ca1"))
IIterator<ABI::Windows::Media::Effects::IAudioEffectDefinition*> : IIterator_impl<ABI::Windows::Media::Effects::IAudioEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Effects.IAudioEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Effects::IAudioEffectDefinition*> __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t;
#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#define DEF___FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("607a20bf-32b4-5b8e-a793-3024f8d3582a"))
IIterable<ABI::Windows::Media::Effects::IAudioEffectDefinition*> : IIterable_impl<ABI::Windows::Media::Effects::IAudioEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Effects.IAudioEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Effects::IAudioEffectDefinition*> __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t;
#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoEffectDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition ABI::Windows::Media::Effects::IVideoEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE
#define DEF___FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9d82379d-4958-558e-a155-3a809bb16c04"))
IIterator<ABI::Windows::Media::Effects::IVideoEffectDefinition*> : IIterator_impl<ABI::Windows::Media::Effects::IVideoEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Effects.IVideoEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Effects::IVideoEffectDefinition*> __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_t;
#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE
#define DEF___FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb567f6f-b014-513d-99cd-f16c226c3c41"))
IIterable<ABI::Windows::Media::Effects::IVideoEffectDefinition*> : IIterable_impl<ABI::Windows::Media::Effects::IVideoEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Effects.IVideoEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Effects::IVideoEffectDefinition*> __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_t;
#define __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6d1f700-49c2-52ae-8154-826f9908773c"))
IMap<HSTRING, HSTRING> : IMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, HSTRING> __FIMap_2_HSTRING_HSTRING_t;
#define __FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7fe52e32-867c-52a3-b3b7-d4dd4d573794"))
IVectorView<ABI::Windows::Media::Editing::BackgroundAudioTrack*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::BackgroundAudioTrack*, ABI::Windows::Media::Editing::IBackgroundAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Editing.BackgroundAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Editing::BackgroundAudioTrack*> __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t;
#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("63d85bd5-4365-5e56-9e40-e7cd3051aebf"))
IVectorView<ABI::Windows::Media::Editing::EmbeddedAudioTrack*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::EmbeddedAudioTrack*, ABI::Windows::Media::Editing::IEmbeddedAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Editing.EmbeddedAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Editing::EmbeddedAudioTrack*> __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_t;
#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eabf18a6-f438-53b0-be3c-af21e67bc8c7"))
IVectorView<ABI::Windows::Media::Editing::MediaClip*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaClip*, ABI::Windows::Media::Editing::IMediaClip*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Editing.MediaClip>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Editing::MediaClip*> __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_t;
#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6bba8a56-2f18-5a68-83cd-9aa9d7f8f757"))
IVectorView<ABI::Windows::Media::Editing::MediaOverlay*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaOverlay*, ABI::Windows::Media::Editing::IMediaOverlay*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Editing.MediaOverlay>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Editing::MediaOverlay*> __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_t;
#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8bcc3ff4-6139-52c1-9669-53da5033185e"))
IVectorView<ABI::Windows::Media::Editing::MediaOverlayLayer*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaOverlayLayer*, ABI::Windows::Media::Editing::IMediaOverlayLayer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Editing.MediaOverlayLayer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Editing::MediaOverlayLayer*> __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_t;
#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("de9e6a7f-d28e-5ef1-916a-efa880b489d1"))
IVectorView<ABI::Windows::Media::Effects::IAudioEffectDefinition*> : IVectorView_impl<ABI::Windows::Media::Effects::IAudioEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Effects.IAudioEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Effects::IAudioEffectDefinition*> __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t;
#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a44b4f8e-7ca3-53b1-b68b-098baf45c73e"))
IVectorView<ABI::Windows::Media::Effects::IVideoEffectDefinition*> : IVectorView_impl<ABI::Windows::Media::Effects::IVideoEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Effects.IVideoEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Effects::IVideoEffectDefinition*> __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_t;
#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#define DEF___FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("56d62145-6978-5eba-83c9-fc4ad6d443bf"))
IVector<ABI::Windows::Media::Editing::BackgroundAudioTrack*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::BackgroundAudioTrack*, ABI::Windows::Media::Editing::IBackgroundAudioTrack*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Editing.BackgroundAudioTrack>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Editing::BackgroundAudioTrack*> __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t;
#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CEditing__CMediaClip_USE
#define DEF___FIVector_1_Windows__CMedia__CEditing__CMediaClip_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e805688e-3508-57f3-bf95-617f2d7a6f1a"))
IVector<ABI::Windows::Media::Editing::MediaClip*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaClip*, ABI::Windows::Media::Editing::IMediaClip*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Editing.MediaClip>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Editing::MediaClip*> __FIVector_1_Windows__CMedia__CEditing__CMediaClip_t;
#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CEditing__CMediaClip_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CEditing__CMediaClip_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_USE
#define DEF___FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5b913ec8-ed52-5b2c-a710-52c6e7ab3cb2"))
IVector<ABI::Windows::Media::Editing::MediaOverlay*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaOverlay*, ABI::Windows::Media::Editing::IMediaOverlay*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Editing.MediaOverlay>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Editing::MediaOverlay*> __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_t;
#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE
#define DEF___FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6564eb2c-2210-5dfc-a84c-266c4349c2c6"))
IVector<ABI::Windows::Media::Editing::MediaOverlayLayer*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Editing::MediaOverlayLayer*, ABI::Windows::Media::Editing::IMediaOverlayLayer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Editing.MediaOverlayLayer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Editing::MediaOverlayLayer*> __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_t;
#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#define DEF___FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2828a982-d849-5fc9-84ce-f9a4b3b4d341"))
IVector<ABI::Windows::Media::Effects::IAudioEffectDefinition*> : IVector_impl<ABI::Windows::Media::Effects::IAudioEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Effects.IAudioEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Effects::IAudioEffectDefinition*> __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t;
#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE
#define DEF___FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7ca161ed-e201-5615-aacc-25348564f0b3"))
IVector<ABI::Windows::Media::Effects::IVideoEffectDefinition*> : IVector_impl<ABI::Windows::Media::Effects::IVideoEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Effects.IVideoEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Effects::IVideoEffectDefinition*> __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_t;
#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DSurface;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                class MediaStreamSource;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IMediaStreamSource;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource ABI::Windows::Media::Core::IMediaStreamSource

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoCompositorDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition ABI::Windows::Media::Effects::IVideoCompositorDefinition

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class AudioEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAudioEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties ABI::Windows::Media::MediaProperties::IAudioEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class VideoEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties ABI::Windows::Media::MediaProperties::IVideoEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__

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
            class StorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                typedef enum MediaTrimmingPreference : int MediaTrimmingPreference;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                typedef enum VideoFramePrecision : int VideoFramePrecision;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Editing.MediaTrimmingPreference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                enum MediaTrimmingPreference : int
                {
                    MediaTrimmingPreference_Fast = 0,
                    MediaTrimmingPreference_Precise = 1,
                };
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Editing.VideoFramePrecision
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                enum VideoFramePrecision : int
                {
                    VideoFramePrecision_NearestFrame = 0,
                    VideoFramePrecision_NearestKeyFrame = 1,
                };
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IBackgroundAudioTrack
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.BackgroundAudioTrack
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IBackgroundAudioTrack[] = L"Windows.Media.Editing.IBackgroundAudioTrack";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("4b91b3bd-9e21-4266-a9c2-67dd011a2357")
                IBackgroundAudioTrack : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TrimTimeFromStart(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TrimTimeFromStart(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrimTimeFromEnd(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TrimTimeFromEnd(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrimmedDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserData(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Delay(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Delay(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Volume(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Volume(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clone(
                        ABI::Windows::Media::Editing::IBackgroundAudioTrack** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioEncodingProperties(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AudioEffectDefinitions(
                        __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundAudioTrack = __uuidof(IBackgroundAudioTrack);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IBackgroundAudioTrackStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.BackgroundAudioTrack
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IBackgroundAudioTrackStatics[] = L"Windows.Media.Editing.IBackgroundAudioTrackStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("d9b1c0d7-d018-42a8-a559-cb4d9e97e664")
                IBackgroundAudioTrackStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromEmbeddedAudioTrack(
                        ABI::Windows::Media::Editing::IEmbeddedAudioTrack* embeddedAudioTrack,
                        ABI::Windows::Media::Editing::IBackgroundAudioTrack** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromFileAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundAudioTrackStatics = __uuidof(IBackgroundAudioTrackStatics);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IEmbeddedAudioTrack
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.EmbeddedAudioTrack
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IEmbeddedAudioTrack[] = L"Windows.Media.Editing.IEmbeddedAudioTrack";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("55ee5a7a-2d30-3fba-a190-4f1a6454f88f")
                IEmbeddedAudioTrack : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAudioEncodingProperties(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEmbeddedAudioTrack = __uuidof(IEmbeddedAudioTrack);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaClip
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaClip
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaClip[] = L"Windows.Media.Editing.IMediaClip";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("53f25366-5fba-3ea4-8693-24761811140a")
                IMediaClip : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TrimTimeFromStart(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TrimTimeFromStart(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrimTimeFromEnd(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TrimTimeFromEnd(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrimmedDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserData(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clone(
                        ABI::Windows::Media::Editing::IMediaClip** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartTimeInComposition(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndTimeInComposition(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EmbeddedAudioTracks(
                        __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedEmbeddedAudioTrackIndex(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SelectedEmbeddedAudioTrackIndex(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Volume(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Volume(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVideoEncodingProperties(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AudioEffectDefinitions(
                        __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideoEffectDefinitions(
                        __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaClip = __uuidof(IMediaClip);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaClip;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaClipStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaClip
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaClipStatics[] = L"Windows.Media.Editing.IMediaClipStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("fa402b68-928f-43c4-bc6e-783a1a359656")
                IMediaClipStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromColor(
                        ABI::Windows::UI::Color color,
                        ABI::Windows::Foundation::TimeSpan originalDuration,
                        ABI::Windows::Media::Editing::IMediaClip** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromFileAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromImageFileAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Foundation::TimeSpan originalDuration,
                        __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaClipStatics = __uuidof(IMediaClipStatics);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaClipStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaClip
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaClipStatics2[] = L"Windows.Media.Editing.IMediaClipStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("5b1dd7b3-854e-4d9b-877d-4774a556cd12")
                IMediaClipStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromSurface(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface* surface,
                        ABI::Windows::Foundation::TimeSpan originalDuration,
                        ABI::Windows::Media::Editing::IMediaClip** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaClipStatics2 = __uuidof(IMediaClipStatics2);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaComposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaComposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaComposition[] = L"Windows.Media.Editing.IMediaComposition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("2e06e605-dc71-41d6-b837-2d2bc14a2947")
                IMediaComposition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Clips(
                        __FIVector_1_Windows__CMedia__CEditing__CMediaClip** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundAudioTracks(
                        __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserData(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clone(
                        ABI::Windows::Media::Editing::IMediaComposition** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetThumbnailAsync(
                        ABI::Windows::Foundation::TimeSpan timeFromStart,
                        INT32 scaledWidth,
                        INT32 scaledHeight,
                        ABI::Windows::Media::Editing::VideoFramePrecision framePrecision,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetThumbnailsAsync(
                        __FIIterable_1_Windows__CFoundation__CTimeSpan* timesFromStart,
                        INT32 scaledWidth,
                        INT32 scaledHeight,
                        ABI::Windows::Media::Editing::VideoFramePrecision framePrecision,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RenderToFileAsync(
                        ABI::Windows::Storage::IStorageFile* destination,
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RenderToFileWithTrimmingPreferenceAsync(
                        ABI::Windows::Storage::IStorageFile* destination,
                        ABI::Windows::Media::Editing::MediaTrimmingPreference trimmingPreference,
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RenderToFileWithProfileAsync(
                        ABI::Windows::Storage::IStorageFile* destination,
                        ABI::Windows::Media::Editing::MediaTrimmingPreference trimmingPreference,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile* encodingProfile,
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDefaultEncodingProfile(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GenerateMediaStreamSource(
                        ABI::Windows::Media::Core::IMediaStreamSource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GenerateMediaStreamSourceWithProfile(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile* encodingProfile,
                        ABI::Windows::Media::Core::IMediaStreamSource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GeneratePreviewMediaStreamSource(
                        INT32 scaledWidth,
                        INT32 scaledHeight,
                        ABI::Windows::Media::Core::IMediaStreamSource** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaComposition = __uuidof(IMediaComposition);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaComposition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaComposition2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaComposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaComposition2[] = L"Windows.Media.Editing.IMediaComposition2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("a59e5372-2366-492c-bec8-e6dfba6d0281")
                IMediaComposition2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OverlayLayers(
                        __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaComposition2 = __uuidof(IMediaComposition2);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaCompositionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaComposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaCompositionStatics[] = L"Windows.Media.Editing.IMediaCompositionStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("87a08f04-e32a-45ce-8f66-a30df0766224")
                IMediaCompositionStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE LoadAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaCompositionStatics = __uuidof(IMediaCompositionStatics);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaOverlay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaOverlay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaOverlay[] = L"Windows.Media.Editing.IMediaOverlay";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("a902ae5d-7869-4830-8ab1-94dc01c05fa4")
                IMediaOverlay : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Position(
                        ABI::Windows::Foundation::Rect value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Delay(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Delay(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Opacity(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Opacity(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clone(
                        ABI::Windows::Media::Editing::IMediaOverlay** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Clip(
                        ABI::Windows::Media::Editing::IMediaClip** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AudioEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AudioEnabled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaOverlay = __uuidof(IMediaOverlay);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaOverlayFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaOverlay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaOverlayFactory[] = L"Windows.Media.Editing.IMediaOverlayFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("b584828a-6188-4f8f-a2e0-aa552d598e18")
                IMediaOverlayFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Media::Editing::IMediaClip* clip,
                        ABI::Windows::Media::Editing::IMediaOverlay** mediaOverlay
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithPositionAndOpacity(
                        ABI::Windows::Media::Editing::IMediaClip* clip,
                        ABI::Windows::Foundation::Rect position,
                        DOUBLE opacity,
                        ABI::Windows::Media::Editing::IMediaOverlay** mediaOverlay
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaOverlayFactory = __uuidof(IMediaOverlayFactory);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaOverlayLayer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaOverlayLayer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaOverlayLayer[] = L"Windows.Media.Editing.IMediaOverlayLayer";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("a6d9ba57-eeda-46c6-bbe5-e398c84168ac")
                IMediaOverlayLayer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Clone(
                        ABI::Windows::Media::Editing::IMediaOverlayLayer** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Overlays(
                        __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CustomCompositorDefinition(
                        ABI::Windows::Media::Effects::IVideoCompositorDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaOverlayLayer = __uuidof(IMediaOverlayLayer);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaOverlayLayerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaOverlayLayer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaOverlayLayerFactory[] = L"Windows.Media.Editing.IMediaOverlayLayerFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                MIDL_INTERFACE("947cb473-a39e-4362-abbf-9f8b5070a062")
                IMediaOverlayLayerFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithCompositorDefinition(
                        ABI::Windows::Media::Effects::IVideoCompositorDefinition* compositorDefinition,
                        ABI::Windows::Media::Editing::IMediaOverlayLayer** mediaOverlayLayer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaOverlayLayerFactory = __uuidof(IMediaOverlayLayerFactory);
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.BackgroundAudioTrack
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Editing.IBackgroundAudioTrackStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IBackgroundAudioTrack ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_BackgroundAudioTrack_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_BackgroundAudioTrack_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_BackgroundAudioTrack[] = L"Windows.Media.Editing.BackgroundAudioTrack";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.EmbeddedAudioTrack
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IEmbeddedAudioTrack ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_EmbeddedAudioTrack_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_EmbeddedAudioTrack_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_EmbeddedAudioTrack[] = L"Windows.Media.Editing.EmbeddedAudioTrack";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.MediaClip
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Editing.IMediaClipStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Editing.IMediaClipStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IMediaClip ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_MediaClip_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_MediaClip_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_MediaClip[] = L"Windows.Media.Editing.MediaClip";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.MediaComposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Editing.IMediaCompositionStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IMediaComposition ** Default Interface **
 *    Windows.Media.Editing.IMediaComposition2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_MediaComposition_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_MediaComposition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_MediaComposition[] = L"Windows.Media.Editing.MediaComposition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.MediaOverlay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Editing.IMediaOverlayFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IMediaOverlay ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_MediaOverlay_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_MediaOverlay_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_MediaOverlay[] = L"Windows.Media.Editing.MediaOverlay";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.MediaOverlayLayer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Editing.IMediaOverlayLayerFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IMediaOverlayLayer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_MediaOverlayLayer_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_MediaOverlayLayer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_MediaOverlayLayer[] = L"Windows.Media.Editing.MediaOverlayLayer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaClip __x_ABI_CWindows_CMedia_CEditing_CIMediaClip;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2 __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2 __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CImaging__CImageStream __FIIterator_1_Windows__CGraphics__CImaging__CImageStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CImaging__CImageStream;

typedef struct __FIIterator_1_Windows__CGraphics__CImaging__CImageStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CImaging__CImageStream* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CImaging__CImageStreamVtbl;

interface __FIIterator_1_Windows__CGraphics__CImaging__CImageStream
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CImaging__CImageStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CImageStream_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CImaging__CImageStream __FIIterable_1_Windows__CGraphics__CImaging__CImageStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CImaging__CImageStream;

typedef struct __FIIterable_1_Windows__CGraphics__CImaging__CImageStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CImaging__CImageStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CImaging__CImageStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CImaging__CImageStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CImaging__CImageStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CImaging__CImageStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CImaging__CImageStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CImaging__CImageStream* This,
        __FIIterator_1_Windows__CGraphics__CImaging__CImageStream** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CImaging__CImageStreamVtbl;

interface __FIIterable_1_Windows__CGraphics__CImaging__CImageStream
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CImaging__CImageStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CImaging__CImageStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CImaging__CImageStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CImaging__CImageStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CImaging__CImageStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CImaging__CImageStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CImaging__CImageStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CImaging__CImageStream_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream;

typedef struct __FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl;

interface __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        __FIVectorView_1_Windows__CGraphics__CImaging__CImageStream** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStreamVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CImageStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClipVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClipVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClipVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClipVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip* This,
        __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClipVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClipVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaCompositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaCompositionVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaCompositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaCompositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition* This,
        __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaCompositionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaCompositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CEditing__CMediaComposition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason;

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        enum __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double* asyncInfo,
        DOUBLE progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CTimeSpan __FIIterator_1_Windows__CFoundation__CTimeSpan;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CTimeSpan;

typedef struct __FIIterator_1_Windows__CFoundation__CTimeSpanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CTimeSpan* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CTimeSpanVtbl;

interface __FIIterator_1_Windows__CFoundation__CTimeSpan
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CTimeSpanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CTimeSpan_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CTimeSpan __FIIterable_1_Windows__CFoundation__CTimeSpan;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CTimeSpan;

typedef struct __FIIterable_1_Windows__CFoundation__CTimeSpanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CTimeSpan* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CTimeSpan* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CTimeSpan* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CTimeSpan* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CTimeSpan* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CTimeSpan* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CTimeSpan* This,
        __FIIterator_1_Windows__CFoundation__CTimeSpan** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CTimeSpanVtbl;

interface __FIIterable_1_Windows__CFoundation__CTimeSpan
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CTimeSpanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CTimeSpan_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CTimeSpan_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CTimeSpan_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CTimeSpan_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CTimeSpan_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CTimeSpan_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CTimeSpan_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

typedef struct __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl;

interface __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

typedef struct __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __FIIterator_1_Windows__CMedia__CEditing__CBackgroundAudioTrack** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl;

interface __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack;

typedef struct __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl;

interface __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack;

typedef struct __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        __FIIterator_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl;

interface __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEditing__CMediaClip __FIIterator_1_Windows__CMedia__CEditing__CMediaClip;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEditing__CMediaClip;

typedef struct __FIIterator_1_Windows__CMedia__CEditing__CMediaClipVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEditing__CMediaClipVtbl;

interface __FIIterator_1_Windows__CMedia__CEditing__CMediaClip
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEditing__CMediaClipVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaClip_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEditing__CMediaClip __FIIterable_1_Windows__CMedia__CEditing__CMediaClip;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEditing__CMediaClip;

typedef struct __FIIterable_1_Windows__CMedia__CEditing__CMediaClipVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEditing__CMediaClip* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEditing__CMediaClip* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEditing__CMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEditing__CMediaClip* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEditing__CMediaClip* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEditing__CMediaClip* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEditing__CMediaClip* This,
        __FIIterator_1_Windows__CMedia__CEditing__CMediaClip** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEditing__CMediaClipVtbl;

interface __FIIterable_1_Windows__CMedia__CEditing__CMediaClip
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEditing__CMediaClipVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaClip_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaClip_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaClip_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaClip_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaClip_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaClip_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaClip_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay;

typedef struct __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayVtbl;

interface __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay;

typedef struct __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlay** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayVtbl;

interface __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer;

typedef struct __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl;

interface __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer;

typedef struct __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        __FIIterator_1_Windows__CMedia__CEditing__CMediaOverlayLayer** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl;

interface __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

typedef struct __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl;

interface __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

typedef struct __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl;

interface __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition;

typedef struct __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl;

interface __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition;

typedef struct __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        __FIIterator_1_Windows__CMedia__CEffects__CIVideoEffectDefinition** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl;

interface __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_HSTRING __FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_HSTRING;

typedef struct __FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_HSTRING* This);

    END_INTERFACE
} __FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_HSTRING_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_HSTRING_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

typedef struct __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl;

interface __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack;

typedef struct __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl;

interface __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEditing__CMediaClip;

typedef struct __FIVectorView_1_Windows__CMedia__CEditing__CMediaClipVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEditing__CMediaClipVtbl;

interface __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEditing__CMediaClipVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay;

typedef struct __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayVtbl;

interface __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer;

typedef struct __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl;

interface __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

typedef struct __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl;

interface __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition;

typedef struct __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl;

interface __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack;

typedef struct __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __FIVectorView_1_Windows__CMedia__CEditing__CBackgroundAudioTrack** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl;

interface __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CEditing__CMediaClip __FIVector_1_Windows__CMedia__CEditing__CMediaClip;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CEditing__CMediaClip;

typedef struct __FIVector_1_Windows__CMedia__CEditing__CMediaClipVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        __FIVectorView_1_Windows__CMedia__CEditing__CMediaClip** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CEditing__CMediaClip* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CEditing__CMediaClipVtbl;

interface __FIVector_1_Windows__CMedia__CEditing__CMediaClip
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CEditing__CMediaClipVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaClip_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CEditing__CMediaClip_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CEditing__CMediaOverlay;

typedef struct __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlay** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlay* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayVtbl;

interface __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CEditing__CMediaOverlay_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer;

typedef struct __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        __FIVectorView_1_Windows__CMedia__CEditing__CMediaOverlayLayer** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl;

interface __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

typedef struct __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl;

interface __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition;

typedef struct __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        __FIVectorView_1_Windows__CMedia__CEffects__CIVideoEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl;

interface __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CMedia_CEditing_CMediaTrimmingPreference __x_ABI_CWindows_CMedia_CEditing_CMediaTrimmingPreference;

typedef enum __x_ABI_CWindows_CMedia_CEditing_CVideoFramePrecision __x_ABI_CWindows_CMedia_CEditing_CVideoFramePrecision;

/*
 *
 * Struct Windows.Media.Editing.MediaTrimmingPreference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CEditing_CMediaTrimmingPreference
{
    MediaTrimmingPreference_Fast = 0,
    MediaTrimmingPreference_Precise = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Editing.VideoFramePrecision
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CEditing_CVideoFramePrecision
{
    VideoFramePrecision_NearestFrame = 0,
    VideoFramePrecision_NearestKeyFrame = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IBackgroundAudioTrack
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.BackgroundAudioTrack
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IBackgroundAudioTrack[] = L"Windows.Media.Editing.IBackgroundAudioTrack";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TrimTimeFromStart)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_TrimTimeFromStart)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_TrimTimeFromEnd)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_TrimTimeFromEnd)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_OriginalDuration)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_TrimmedDuration)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_UserData)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* put_Delay)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_Delay)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_Volume)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Volume)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** value);
    HRESULT (STDMETHODCALLTYPE* GetAudioEncodingProperties)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_AudioEffectDefinitions)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack* This,
        __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_get_TrimTimeFromStart(This, value) \
    ((This)->lpVtbl->get_TrimTimeFromStart(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_put_TrimTimeFromStart(This, value) \
    ((This)->lpVtbl->put_TrimTimeFromStart(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_get_TrimTimeFromEnd(This, value) \
    ((This)->lpVtbl->get_TrimTimeFromEnd(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_put_TrimTimeFromEnd(This, value) \
    ((This)->lpVtbl->put_TrimTimeFromEnd(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_get_OriginalDuration(This, value) \
    ((This)->lpVtbl->get_OriginalDuration(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_get_TrimmedDuration(This, value) \
    ((This)->lpVtbl->get_TrimmedDuration(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_get_UserData(This, value) \
    ((This)->lpVtbl->get_UserData(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_put_Delay(This, value) \
    ((This)->lpVtbl->put_Delay(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_get_Delay(This, value) \
    ((This)->lpVtbl->get_Delay(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_put_Volume(This, value) \
    ((This)->lpVtbl->put_Volume(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_get_Volume(This, value) \
    ((This)->lpVtbl->get_Volume(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_Clone(This, value) \
    ((This)->lpVtbl->Clone(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_GetAudioEncodingProperties(This, value) \
    ((This)->lpVtbl->GetAudioEncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_get_AudioEffectDefinitions(This, value) \
    ((This)->lpVtbl->get_AudioEffectDefinitions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IBackgroundAudioTrackStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.BackgroundAudioTrack
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IBackgroundAudioTrackStatics[] = L"Windows.Media.Editing.IBackgroundAudioTrackStatics";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromEmbeddedAudioTrack)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics* This,
        __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* embeddedAudioTrack,
        __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrack** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromFileAsync)(__x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CMedia__CEditing__CBackgroundAudioTrack** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_CreateFromEmbeddedAudioTrack(This, embeddedAudioTrack, value) \
    ((This)->lpVtbl->CreateFromEmbeddedAudioTrack(This, embeddedAudioTrack, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_CreateFromFileAsync(This, file, operation) \
    ((This)->lpVtbl->CreateFromFileAsync(This, file, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIBackgroundAudioTrackStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IEmbeddedAudioTrack
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.EmbeddedAudioTrack
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IEmbeddedAudioTrack[] = L"Windows.Media.Editing.IEmbeddedAudioTrack";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAudioEncodingProperties)(__x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrackVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_GetAudioEncodingProperties(This, value) \
    ((This)->lpVtbl->GetAudioEncodingProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIEmbeddedAudioTrack_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaClip
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaClip
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaClip[] = L"Windows.Media.Editing.IMediaClip";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaClipVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TrimTimeFromStart)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_TrimTimeFromStart)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_TrimTimeFromEnd)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_TrimTimeFromEnd)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_OriginalDuration)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_TrimmedDuration)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_UserData)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** result);
    HRESULT (STDMETHODCALLTYPE* get_StartTimeInComposition)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_EndTimeInComposition)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_EmbeddedAudioTracks)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        __FIVectorView_1_Windows__CMedia__CEditing__CEmbeddedAudioTrack** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedEmbeddedAudioTrackIndex)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_SelectedEmbeddedAudioTrackIndex)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* put_Volume)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Volume)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* GetVideoEncodingProperties)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_AudioEffectDefinitions)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoEffectDefinitions)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClip* This,
        __FIVector_1_Windows__CMedia__CEffects__CIVideoEffectDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaClipVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaClip
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaClipVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_TrimTimeFromStart(This, value) \
    ((This)->lpVtbl->get_TrimTimeFromStart(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_put_TrimTimeFromStart(This, value) \
    ((This)->lpVtbl->put_TrimTimeFromStart(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_TrimTimeFromEnd(This, value) \
    ((This)->lpVtbl->get_TrimTimeFromEnd(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_put_TrimTimeFromEnd(This, value) \
    ((This)->lpVtbl->put_TrimTimeFromEnd(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_OriginalDuration(This, value) \
    ((This)->lpVtbl->get_OriginalDuration(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_TrimmedDuration(This, value) \
    ((This)->lpVtbl->get_TrimmedDuration(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_UserData(This, value) \
    ((This)->lpVtbl->get_UserData(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_Clone(This, result) \
    ((This)->lpVtbl->Clone(This, result))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_StartTimeInComposition(This, value) \
    ((This)->lpVtbl->get_StartTimeInComposition(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_EndTimeInComposition(This, value) \
    ((This)->lpVtbl->get_EndTimeInComposition(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_EmbeddedAudioTracks(This, value) \
    ((This)->lpVtbl->get_EmbeddedAudioTracks(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_SelectedEmbeddedAudioTrackIndex(This, value) \
    ((This)->lpVtbl->get_SelectedEmbeddedAudioTrackIndex(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_put_SelectedEmbeddedAudioTrackIndex(This, value) \
    ((This)->lpVtbl->put_SelectedEmbeddedAudioTrackIndex(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_put_Volume(This, value) \
    ((This)->lpVtbl->put_Volume(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_Volume(This, value) \
    ((This)->lpVtbl->get_Volume(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_GetVideoEncodingProperties(This, value) \
    ((This)->lpVtbl->GetVideoEncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_AudioEffectDefinitions(This, value) \
    ((This)->lpVtbl->get_AudioEffectDefinitions(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClip_get_VideoEffectDefinitions(This, value) \
    ((This)->lpVtbl->get_VideoEffectDefinitions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaClip;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClip_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaClipStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaClip
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaClipStatics[] = L"Windows.Media.Editing.IMediaClipStatics";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromColor)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This,
        struct __x_ABI_CWindows_CUI_CColor color,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan originalDuration,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromFileAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFromImageFileAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan originalDuration,
        __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaClip** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_CreateFromColor(This, color, originalDuration, value) \
    ((This)->lpVtbl->CreateFromColor(This, color, originalDuration, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_CreateFromFileAsync(This, file, operation) \
    ((This)->lpVtbl->CreateFromFileAsync(This, file, operation))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_CreateFromImageFileAsync(This, file, originalDuration, operation) \
    ((This)->lpVtbl->CreateFromImageFileAsync(This, file, originalDuration, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaClipStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaClip
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaClipStatics2[] = L"Windows.Media.Editing.IMediaClipStatics2";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromSurface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* surface,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan originalDuration,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_CreateFromSurface(This, surface, originalDuration, value) \
    ((This)->lpVtbl->CreateFromSurface(This, surface, originalDuration, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaClipStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaComposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaComposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaComposition[] = L"Windows.Media.Editing.IMediaComposition";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Clips)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __FIVector_1_Windows__CMedia__CEditing__CMediaClip** value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundAudioTracks)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __FIVector_1_Windows__CMedia__CEditing__CBackgroundAudioTrack** value);
    HRESULT (STDMETHODCALLTYPE* get_UserData)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition** result);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* GetThumbnailAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan timeFromStart,
        INT32 scaledWidth,
        INT32 scaledHeight,
        enum __x_ABI_CWindows_CMedia_CEditing_CVideoFramePrecision framePrecision,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream** operation);
    HRESULT (STDMETHODCALLTYPE* GetThumbnailsAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __FIIterable_1_Windows__CFoundation__CTimeSpan* timesFromStart,
        INT32 scaledWidth,
        INT32 scaledHeight,
        enum __x_ABI_CWindows_CMedia_CEditing_CVideoFramePrecision framePrecision,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CGraphics__CImaging__CImageStream** operation);
    HRESULT (STDMETHODCALLTYPE* RenderToFileAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* destination,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double** operation);
    HRESULT (STDMETHODCALLTYPE* RenderToFileWithTrimmingPreferenceAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* destination,
        enum __x_ABI_CWindows_CMedia_CEditing_CMediaTrimmingPreference trimmingPreference,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double** operation);
    HRESULT (STDMETHODCALLTYPE* RenderToFileWithProfileAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* destination,
        enum __x_ABI_CWindows_CMedia_CEditing_CMediaTrimmingPreference trimmingPreference,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* encodingProfile,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CTranscoding__CTranscodeFailureReason_double** operation);
    HRESULT (STDMETHODCALLTYPE* CreateDefaultEncodingProfile)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* GenerateMediaStreamSource)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource** value);
    HRESULT (STDMETHODCALLTYPE* GenerateMediaStreamSourceWithProfile)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* encodingProfile,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource** value);
    HRESULT (STDMETHODCALLTYPE* GeneratePreviewMediaStreamSource)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition* This,
        INT32 scaledWidth,
        INT32 scaledHeight,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_get_Clips(This, value) \
    ((This)->lpVtbl->get_Clips(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_get_BackgroundAudioTracks(This, value) \
    ((This)->lpVtbl->get_BackgroundAudioTracks(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_get_UserData(This, value) \
    ((This)->lpVtbl->get_UserData(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_Clone(This, result) \
    ((This)->lpVtbl->Clone(This, result))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_SaveAsync(This, file, operation) \
    ((This)->lpVtbl->SaveAsync(This, file, operation))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_GetThumbnailAsync(This, timeFromStart, scaledWidth, scaledHeight, framePrecision, operation) \
    ((This)->lpVtbl->GetThumbnailAsync(This, timeFromStart, scaledWidth, scaledHeight, framePrecision, operation))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_GetThumbnailsAsync(This, timesFromStart, scaledWidth, scaledHeight, framePrecision, operation) \
    ((This)->lpVtbl->GetThumbnailsAsync(This, timesFromStart, scaledWidth, scaledHeight, framePrecision, operation))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_RenderToFileAsync(This, destination, operation) \
    ((This)->lpVtbl->RenderToFileAsync(This, destination, operation))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_RenderToFileWithTrimmingPreferenceAsync(This, destination, trimmingPreference, operation) \
    ((This)->lpVtbl->RenderToFileWithTrimmingPreferenceAsync(This, destination, trimmingPreference, operation))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_RenderToFileWithProfileAsync(This, destination, trimmingPreference, encodingProfile, operation) \
    ((This)->lpVtbl->RenderToFileWithProfileAsync(This, destination, trimmingPreference, encodingProfile, operation))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_CreateDefaultEncodingProfile(This, value) \
    ((This)->lpVtbl->CreateDefaultEncodingProfile(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_GenerateMediaStreamSource(This, value) \
    ((This)->lpVtbl->GenerateMediaStreamSource(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_GenerateMediaStreamSourceWithProfile(This, encodingProfile, value) \
    ((This)->lpVtbl->GenerateMediaStreamSourceWithProfile(This, encodingProfile, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_GeneratePreviewMediaStreamSource(This, scaledWidth, scaledHeight, value) \
    ((This)->lpVtbl->GeneratePreviewMediaStreamSource(This, scaledWidth, scaledHeight, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaComposition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaComposition2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaComposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaComposition2[] = L"Windows.Media.Editing.IMediaComposition2";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OverlayLayers)(__x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2* This,
        __FIVector_1_Windows__CMedia__CEditing__CMediaOverlayLayer** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2Vtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_get_OverlayLayers(This, value) \
    ((This)->lpVtbl->get_OverlayLayers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaComposition2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaCompositionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaComposition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaCompositionStatics[] = L"Windows.Media.Editing.IMediaCompositionStatics";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadAsync)(__x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CMedia__CEditing__CMediaComposition** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_LoadAsync(This, file, operation) \
    ((This)->lpVtbl->LoadAsync(This, file, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaCompositionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaOverlay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaOverlay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaOverlay[] = L"Windows.Media.Editing.IMediaOverlay";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* put_Position)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        struct __x_ABI_CWindows_CFoundation_CRect value);
    HRESULT (STDMETHODCALLTYPE* put_Delay)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_Delay)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Opacity)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Opacity)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** result);
    HRESULT (STDMETHODCALLTYPE* get_Clip)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip** value);
    HRESULT (STDMETHODCALLTYPE* get_AudioEnabled)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AudioEnabled)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_put_Position(This, value) \
    ((This)->lpVtbl->put_Position(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_put_Delay(This, value) \
    ((This)->lpVtbl->put_Delay(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_get_Delay(This, value) \
    ((This)->lpVtbl->get_Delay(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_get_Opacity(This, value) \
    ((This)->lpVtbl->get_Opacity(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_put_Opacity(This, value) \
    ((This)->lpVtbl->put_Opacity(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_Clone(This, result) \
    ((This)->lpVtbl->Clone(This, result))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_get_Clip(This, value) \
    ((This)->lpVtbl->get_Clip(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_get_AudioEnabled(This, value) \
    ((This)->lpVtbl->get_AudioEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_put_AudioEnabled(This, value) \
    ((This)->lpVtbl->put_AudioEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaOverlayFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaOverlay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaOverlayFactory[] = L"Windows.Media.Editing.IMediaOverlayFactory";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip* clip,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** mediaOverlay);
    HRESULT (STDMETHODCALLTYPE* CreateWithPositionAndOpacity)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaClip* clip,
        struct __x_ABI_CWindows_CFoundation_CRect position,
        DOUBLE opacity,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** mediaOverlay);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_Create(This, clip, mediaOverlay) \
    ((This)->lpVtbl->Create(This, clip, mediaOverlay))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_CreateWithPositionAndOpacity(This, clip, position, opacity, mediaOverlay) \
    ((This)->lpVtbl->CreateWithPositionAndOpacity(This, clip, position, opacity, mediaOverlay))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaOverlayLayer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaOverlayLayer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaOverlayLayer[] = L"Windows.Media.Editing.IMediaOverlayLayer";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** result);
    HRESULT (STDMETHODCALLTYPE* get_Overlays)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This,
        __FIVector_1_Windows__CMedia__CEditing__CMediaOverlay** value);
    HRESULT (STDMETHODCALLTYPE* get_CustomCompositorDefinition)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer* This,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_Clone(This, result) \
    ((This)->lpVtbl->Clone(This, result))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_get_Overlays(This, value) \
    ((This)->lpVtbl->get_Overlays(This, value))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_get_CustomCompositorDefinition(This, value) \
    ((This)->lpVtbl->get_CustomCompositorDefinition(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Editing.IMediaOverlayLayerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Editing.MediaOverlayLayer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Editing_IMediaOverlayLayerFactory[] = L"Windows.Media.Editing.IMediaOverlayLayerFactory";
typedef struct __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithCompositorDefinition)(__x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory* This,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* compositorDefinition,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayer** mediaOverlayLayer);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_CreateWithCompositorDefinition(This, compositorDefinition, mediaOverlayLayer) \
    ((This)->lpVtbl->CreateWithCompositorDefinition(This, compositorDefinition, mediaOverlayLayer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlayLayerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.BackgroundAudioTrack
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Editing.IBackgroundAudioTrackStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IBackgroundAudioTrack ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_BackgroundAudioTrack_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_BackgroundAudioTrack_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_BackgroundAudioTrack[] = L"Windows.Media.Editing.BackgroundAudioTrack";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.EmbeddedAudioTrack
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IEmbeddedAudioTrack ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_EmbeddedAudioTrack_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_EmbeddedAudioTrack_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_EmbeddedAudioTrack[] = L"Windows.Media.Editing.EmbeddedAudioTrack";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.MediaClip
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Editing.IMediaClipStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Editing.IMediaClipStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IMediaClip ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_MediaClip_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_MediaClip_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_MediaClip[] = L"Windows.Media.Editing.MediaClip";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.MediaComposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Editing.IMediaCompositionStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IMediaComposition ** Default Interface **
 *    Windows.Media.Editing.IMediaComposition2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_MediaComposition_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_MediaComposition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_MediaComposition[] = L"Windows.Media.Editing.MediaComposition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.MediaOverlay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Editing.IMediaOverlayFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IMediaOverlay ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_MediaOverlay_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_MediaOverlay_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_MediaOverlay[] = L"Windows.Media.Editing.MediaOverlay";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Editing.MediaOverlayLayer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Editing.IMediaOverlayLayerFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Editing.IMediaOverlayLayer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Editing_MediaOverlayLayer_DEFINED
#define RUNTIMECLASS_Windows_Media_Editing_MediaOverlayLayer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Editing_MediaOverlayLayer[] = L"Windows.Media.Editing.MediaOverlayLayer";
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
#endif // __windows2Emedia2Eediting_p_h__

#endif // __windows2Emedia2Eediting_h__
