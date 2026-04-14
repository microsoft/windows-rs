
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
#ifndef __windows2Egraphics2Eimaging_h__
#define __windows2Egraphics2Eimaging_h__
#ifndef __windows2Egraphics2Eimaging_p_h__
#define __windows2Egraphics2Eimaging_p_h__


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
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapBuffer;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer ABI::Windows::Graphics::Imaging::IBitmapBuffer

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapCodecInformation;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation ABI::Windows::Graphics::Imaging::IBitmapCodecInformation

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapDecoder;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder ABI::Windows::Graphics::Imaging::IBitmapDecoder

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapDecoderStatics;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics ABI::Windows::Graphics::Imaging::IBitmapDecoderStatics

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapDecoderStatics2;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2 ABI::Windows::Graphics::Imaging::IBitmapDecoderStatics2

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapEncoder;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder ABI::Windows::Graphics::Imaging::IBitmapEncoder

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapEncoderStatics;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics ABI::Windows::Graphics::Imaging::IBitmapEncoderStatics

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapEncoderStatics2;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2 ABI::Windows::Graphics::Imaging::IBitmapEncoderStatics2

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapEncoderWithSoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap ABI::Windows::Graphics::Imaging::IBitmapEncoderWithSoftwareBitmap

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapFrame;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame ABI::Windows::Graphics::Imaging::IBitmapFrame

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapFrameWithSoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap ABI::Windows::Graphics::Imaging::IBitmapFrameWithSoftwareBitmap

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapProperties;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties ABI::Windows::Graphics::Imaging::IBitmapProperties

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapPropertiesView;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView ABI::Windows::Graphics::Imaging::IBitmapPropertiesView

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapTransform;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform ABI::Windows::Graphics::Imaging::IBitmapTransform

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapTypedValue;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue ABI::Windows::Graphics::Imaging::IBitmapTypedValue

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IBitmapTypedValueFactory;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory ABI::Windows::Graphics::Imaging::IBitmapTypedValueFactory

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface IPixelDataProvider;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider ABI::Windows::Graphics::Imaging::IPixelDataProvider

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface ISoftwareBitmapFactory;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory ABI::Windows::Graphics::Imaging::ISoftwareBitmapFactory

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface ISoftwareBitmapStatics;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics ABI::Windows::Graphics::Imaging::ISoftwareBitmapStatics

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapDecoder;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("aa94d8e9-caef-53f6-823d-91b6e8340510"))
IAsyncOperation<ABI::Windows::Graphics::Imaging::BitmapDecoder*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapDecoder*, ABI::Windows::Graphics::Imaging::IBitmapDecoder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Imaging.BitmapDecoder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Imaging::BitmapDecoder*> __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bb6514f2-3cfb-566f-82bc-60aabd302d53"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::BitmapDecoder*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapDecoder*, ABI::Windows::Graphics::Imaging::IBitmapDecoder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Imaging.BitmapDecoder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::BitmapDecoder*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapEncoder;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("151bd1c5-4675-5af5-a289-001edc66b86a"))
IAsyncOperation<ABI::Windows::Graphics::Imaging::BitmapEncoder*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapEncoder*, ABI::Windows::Graphics::Imaging::IBitmapEncoder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Imaging.BitmapEncoder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Imaging::BitmapEncoder*> __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5df1afc5-478d-55dd-b317-024274062a0d"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::BitmapEncoder*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapEncoder*, ABI::Windows::Graphics::Imaging::IBitmapEncoder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Imaging.BitmapEncoder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::BitmapEncoder*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapFrame;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cb1483d1-1464-5bf9-9346-d537735dfbd6"))
IAsyncOperation<ABI::Windows::Graphics::Imaging::BitmapFrame*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapFrame*, ABI::Windows::Graphics::Imaging::IBitmapFrame*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Imaging.BitmapFrame>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Imaging::BitmapFrame*> __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2817455a-983f-5a06-9fe4-fb9637684320"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::BitmapFrame*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapFrame*, ABI::Windows::Graphics::Imaging::IBitmapFrame*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Imaging.BitmapFrame>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::BitmapFrame*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapPropertySet;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapTypedValue;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("93621bf0-dae9-5f00-94ac-795aa943dca6"))
IKeyValuePair<HSTRING, ABI::Windows::Graphics::Imaging::BitmapTypedValue*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapTypedValue*, ABI::Windows::Graphics::Imaging::IBitmapTypedValue*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Graphics.Imaging.BitmapTypedValue>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Graphics::Imaging::BitmapTypedValue*> __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2ad3fb0c-0656-5302-b504-3153be845161"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Graphics.Imaging.BitmapTypedValue>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05f9430c-2f22-5638-aa89-8c9abcd54ff9"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Graphics.Imaging.BitmapTypedValue>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#define DEF___FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9cda5a9a-8924-5b3b-8b19-894d8da99dde"))
IMapView<HSTRING, ABI::Windows::Graphics::Imaging::BitmapTypedValue*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapTypedValue*, ABI::Windows::Graphics::Imaging::IBitmapTypedValue*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Graphics.Imaging.BitmapTypedValue>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Graphics::Imaging::BitmapTypedValue*> __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t;
#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#define DEF___FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2c70ef8d-5d4c-5185-8db7-fed87728165d"))
IMap<HSTRING, ABI::Windows::Graphics::Imaging::BitmapTypedValue*> : IMap_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapTypedValue*, ABI::Windows::Graphics::Imaging::IBitmapTypedValue*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Windows.Graphics.Imaging.BitmapTypedValue>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, ABI::Windows::Graphics::Imaging::BitmapTypedValue*> __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t;
#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("464ac000-b1f1-5246-8268-912a2593d889"))
IAsyncOperation<ABI::Windows::Graphics::Imaging::BitmapPropertySet*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapPropertySet*, __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Imaging.BitmapPropertySet>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Imaging::BitmapPropertySet*> __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a8325bd7-a3be-5881-9fa7-04ceefb9dc2f"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::BitmapPropertySet*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapPropertySet*, __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Imaging.BitmapPropertySet>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::BitmapPropertySet*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace Graphics {
            namespace Imaging {
                class PixelDataProvider;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8c2dfeb0-6c22-5863-88d8-85c1fbc75697"))
IAsyncOperation<ABI::Windows::Graphics::Imaging::PixelDataProvider*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::PixelDataProvider*, ABI::Windows::Graphics::Imaging::IPixelDataProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Imaging.PixelDataProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Imaging::PixelDataProvider*> __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("37bdf4be-2f39-592c-a4f7-d16a09d2b2db"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::PixelDataProvider*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::PixelDataProvider*, ABI::Windows::Graphics::Imaging::IPixelDataProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Imaging.PixelDataProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::PixelDataProvider*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class SoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c4a10980-714b-5501-8da2-dbdacce70f73"))
IAsyncOperation<ABI::Windows::Graphics::Imaging::SoftwareBitmap*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::SoftwareBitmap*, ABI::Windows::Graphics::Imaging::ISoftwareBitmap*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Imaging.SoftwareBitmap>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Imaging::SoftwareBitmap*> __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b699b653-33ed-5e2d-a75f-02bf90e32619"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::SoftwareBitmap*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::SoftwareBitmap*, ABI::Windows::Graphics::Imaging::ISoftwareBitmap*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Imaging.SoftwareBitmap>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Imaging::SoftwareBitmap*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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
            namespace Imaging {
                class BitmapCodecInformation;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE
#define DEF___FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4ff2b2db-9326-537f-b8dc-4c93d77fbb84"))
IIterator<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*, ABI::Windows::Graphics::Imaging::IBitmapCodecInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Imaging.BitmapCodecInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*> __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_t;
#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE
#define DEF___FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2b6bdb90-a4eb-5142-b582-3ccb1edc5789"))
IIterable<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*, ABI::Windows::Graphics::Imaging::IBitmapCodecInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Imaging.BitmapCodecInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*> __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_t;
#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE */

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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("97dfde96-ff1d-5aa1-863a-90116a31b86b"))
IVectorView<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*, ABI::Windows::Graphics::Imaging::IBitmapCodecInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Imaging.BitmapCodecInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Imaging::BitmapCodecInformation*> __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_t;
#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_USE */

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

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IMemoryBuffer;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer ABI::Windows::Foundation::IMemoryBuffer

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef enum PropertyType : int PropertyType;
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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapAlphaMode : int BitmapAlphaMode;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapBufferAccessMode : int BitmapBufferAccessMode;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapFlip : int BitmapFlip;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapInterpolationMode : int BitmapInterpolationMode;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapPixelFormat : int BitmapPixelFormat;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapRotation : int BitmapRotation;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum ColorManagementMode : int ColorManagementMode;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum ExifOrientationMode : int ExifOrientationMode;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

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
                typedef struct BitmapPlaneDescription BitmapPlaneDescription;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapBuffer;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapProperties;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapPropertiesView;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class BitmapTransform;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapAlphaMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum BitmapAlphaMode : int
                {
                    BitmapAlphaMode_Premultiplied = 0,
                    BitmapAlphaMode_Straight = 1,
                    BitmapAlphaMode_Ignore = 2,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapBufferAccessMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum BitmapBufferAccessMode : int
                {
                    BitmapBufferAccessMode_Read = 0,
                    BitmapBufferAccessMode_ReadWrite = 1,
                    BitmapBufferAccessMode_Write = 2,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapFlip
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum BitmapFlip : int
                {
                    BitmapFlip_None = 0,
                    BitmapFlip_Horizontal = 1,
                    BitmapFlip_Vertical = 2,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapInterpolationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum BitmapInterpolationMode : int
                {
                    BitmapInterpolationMode_NearestNeighbor = 0,
                    BitmapInterpolationMode_Linear = 1,
                    BitmapInterpolationMode_Cubic = 2,
                    BitmapInterpolationMode_Fant = 3,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapPixelFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum BitmapPixelFormat : int
                {
                    BitmapPixelFormat_Unknown = 0,
                    BitmapPixelFormat_Rgba16 = 12,
                    BitmapPixelFormat_Rgba8 = 30,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    BitmapPixelFormat_Gray16 = 57,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    BitmapPixelFormat_Gray8 = 62,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    BitmapPixelFormat_Bgra8 = 87,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    BitmapPixelFormat_Nv12 = 103,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    BitmapPixelFormat_P010 = 104,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    BitmapPixelFormat_Yuy2 = 107,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapRotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum BitmapRotation : int
                {
                    BitmapRotation_None = 0,
                    BitmapRotation_Clockwise90Degrees = 1,
                    BitmapRotation_Clockwise180Degrees = 2,
                    BitmapRotation_Clockwise270Degrees = 3,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.ColorManagementMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum ColorManagementMode : int
                {
                    ColorManagementMode_DoNotColorManage = 0,
                    ColorManagementMode_ColorManageToSRgb = 1,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.ExifOrientationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum ExifOrientationMode : int
                {
                    ExifOrientationMode_IgnoreExifOrientation = 0,
                    ExifOrientationMode_RespectExifOrientation = 1,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.JpegSubsamplingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum JpegSubsamplingMode : int
                {
                    JpegSubsamplingMode_Default = 0,
                    JpegSubsamplingMode_Y4Cb2Cr0 = 1,
                    JpegSubsamplingMode_Y4Cb2Cr2 = 2,
                    JpegSubsamplingMode_Y4Cb4Cr4 = 3,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.PngFilterMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum PngFilterMode : int
                {
                    PngFilterMode_Automatic = 0,
                    PngFilterMode_None = 1,
                    PngFilterMode_Sub = 2,
                    PngFilterMode_Up = 3,
                    PngFilterMode_Average = 4,
                    PngFilterMode_Paeth = 5,
                    PngFilterMode_Adaptive = 6,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.TiffCompressionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                enum TiffCompressionMode : int
                {
                    TiffCompressionMode_Automatic = 0,
                    TiffCompressionMode_None = 1,
                    TiffCompressionMode_Ccitt3 = 2,
                    TiffCompressionMode_Ccitt4 = 3,
                    TiffCompressionMode_Lzw = 4,
                    TiffCompressionMode_Rle = 5,
                    TiffCompressionMode_Zip = 6,
                    TiffCompressionMode_LzwhDifferencing = 7,
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                struct BitmapBounds
                {
                    UINT32 X;
                    UINT32 Y;
                    UINT32 Width;
                    UINT32 Height;
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapPlaneDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                struct BitmapPlaneDescription
                {
                    INT32 StartIndex;
                    INT32 Width;
                    INT32 Height;
                    INT32 Stride;
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapSize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                struct BitmapSize
                {
                    UINT32 Width;
                    UINT32 Height;
                };
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapBuffer
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IMemoryBuffer
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapBuffer[] = L"Windows.Graphics.Imaging.IBitmapBuffer";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("a53e04c4-399c-438c-b28f-a63a6b83d1a1")
                IBitmapBuffer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPlaneCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPlaneDescription(
                        INT32 index,
                        ABI::Windows::Graphics::Imaging::BitmapPlaneDescription* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapBuffer = __uuidof(IBitmapBuffer);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapCodecInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapCodecInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapCodecInformation[] = L"Windows.Graphics.Imaging.IBitmapCodecInformation";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("400caaf2-c4b0-4392-a3b0-6f6f9ba95cb4")
                IBitmapCodecInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CodecId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileExtensions(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MimeTypes(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapCodecInformation = __uuidof(IBitmapCodecInformation);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapDecoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapDecoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapDecoder[] = L"Windows.Graphics.Imaging.IBitmapDecoder";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("acef22ba-1d74-4c91-9dfc-9620745233e6")
                IBitmapDecoder : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapContainerProperties(
                        ABI::Windows::Graphics::Imaging::IBitmapPropertiesView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DecoderInformation(
                        ABI::Windows::Graphics::Imaging::IBitmapCodecInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FrameCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPreviewAsync(
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFrameAsync(
                        UINT32 frameIndex,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapDecoder = __uuidof(IBitmapDecoder);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapDecoderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapDecoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapDecoderStatics[] = L"Windows.Graphics.Imaging.IBitmapDecoderStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("438ccb26-bcef-4e95-bad6-23a822e58d01")
                IBitmapDecoderStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BmpDecoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_JpegDecoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PngDecoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TiffDecoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GifDecoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_JpegXRDecoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IcoDecoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDecoderInformationEnumerator(
                        __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation** decoderInformationEnumerator
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* stream,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithIdAsync(
                        GUID decoderId,
                        ABI::Windows::Storage::Streams::IRandomAccessStream* stream,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapDecoderStatics = __uuidof(IBitmapDecoderStatics);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapDecoderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapDecoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapDecoderStatics2[] = L"Windows.Graphics.Imaging.IBitmapDecoderStatics2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("50ba68ea-99a1-40c4-80d9-aef0dafa6c3f")
                IBitmapDecoderStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HeifDecoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WebpDecoderId(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapDecoderStatics2 = __uuidof(IBitmapDecoderStatics2);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapEncoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapEncoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapEncoder[] = L"Windows.Graphics.Imaging.IBitmapEncoder";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("2bc468e3-e1f8-4b54-95e8-32919551ce62")
                IBitmapEncoder : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EncoderInformation(
                        ABI::Windows::Graphics::Imaging::IBitmapCodecInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapProperties(
                        ABI::Windows::Graphics::Imaging::IBitmapProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapContainerProperties(
                        ABI::Windows::Graphics::Imaging::IBitmapProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsThumbnailGenerated(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsThumbnailGenerated(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GeneratedThumbnailWidth(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_GeneratedThumbnailWidth(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GeneratedThumbnailHeight(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_GeneratedThumbnailHeight(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapTransform(
                        ABI::Windows::Graphics::Imaging::IBitmapTransform** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPixelData(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat pixelFormat,
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode alphaMode,
                        UINT32 width,
                        UINT32 height,
                        DOUBLE dpiX,
                        DOUBLE dpiY,
                        UINT32 pixelsLength,
                        BYTE* pixels
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GoToNextFrameAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GoToNextFrameWithEncodingOptionsAsync(
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* encodingOptions,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FlushAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapEncoder = __uuidof(IBitmapEncoder);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapEncoderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapEncoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapEncoderStatics[] = L"Windows.Graphics.Imaging.IBitmapEncoderStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("a74356a7-a4e4-4eb9-8e40-564de7e1ccb2")
                IBitmapEncoderStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BmpEncoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_JpegEncoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PngEncoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TiffEncoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GifEncoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_JpegXREncoderId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetEncoderInformationEnumerator(
                        __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation** encoderInformationEnumerator
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                        GUID encoderId,
                        ABI::Windows::Storage::Streams::IRandomAccessStream* stream,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithEncodingOptionsAsync(
                        GUID encoderId,
                        ABI::Windows::Storage::Streams::IRandomAccessStream* stream,
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* encodingOptions,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForTranscodingAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* stream,
                        ABI::Windows::Graphics::Imaging::IBitmapDecoder* bitmapDecoder,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForInPlacePropertyEncodingAsync(
                        ABI::Windows::Graphics::Imaging::IBitmapDecoder* bitmapDecoder,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapEncoderStatics = __uuidof(IBitmapEncoderStatics);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapEncoderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapEncoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapEncoderStatics2[] = L"Windows.Graphics.Imaging.IBitmapEncoderStatics2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("33cbc259-fe31-41b1-b812-086d21e87e16")
                IBitmapEncoderStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HeifEncoderId(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapEncoderStatics2 = __uuidof(IBitmapEncoderStatics2);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapEncoderWithSoftwareBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapEncoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapEncoderWithSoftwareBitmap[] = L"Windows.Graphics.Imaging.IBitmapEncoderWithSoftwareBitmap";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("686cd241-4330-4c77-ace4-0334968b1768")
                IBitmapEncoderWithSoftwareBitmap : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetSoftwareBitmap(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap* bitmap
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapEncoderWithSoftwareBitmap = __uuidof(IBitmapEncoderWithSoftwareBitmap);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapFrame[] = L"Windows.Graphics.Imaging.IBitmapFrame";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("72a49a1c-8081-438d-91bc-94ecfc8185c6")
                IBitmapFrame : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetThumbnailAsync(
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapProperties(
                        ABI::Windows::Graphics::Imaging::IBitmapPropertiesView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapPixelFormat(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapAlphaMode(
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DpiX(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DpiY(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PixelWidth(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PixelHeight(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OrientedPixelWidth(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OrientedPixelHeight(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPixelDataAsync(
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPixelDataTransformedAsync(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat pixelFormat,
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode alphaMode,
                        ABI::Windows::Graphics::Imaging::IBitmapTransform* transform,
                        ABI::Windows::Graphics::Imaging::ExifOrientationMode exifOrientationMode,
                        ABI::Windows::Graphics::Imaging::ColorManagementMode colorManagementMode,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapFrame = __uuidof(IBitmapFrame);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Imaging.IBitmapFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapFrameWithSoftwareBitmap[] = L"Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("fe287c9a-420c-4963-87ad-691436e08383")
                IBitmapFrameWithSoftwareBitmap : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetSoftwareBitmapAsync(
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSoftwareBitmapConvertedAsync(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat pixelFormat,
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode alphaMode,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSoftwareBitmapTransformedAsync(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat pixelFormat,
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode alphaMode,
                        ABI::Windows::Graphics::Imaging::IBitmapTransform* transform,
                        ABI::Windows::Graphics::Imaging::ExifOrientationMode exifOrientationMode,
                        ABI::Windows::Graphics::Imaging::ColorManagementMode colorManagementMode,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapFrameWithSoftwareBitmap = __uuidof(IBitmapFrameWithSoftwareBitmap);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Imaging.IBitmapPropertiesView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapProperties[] = L"Windows.Graphics.Imaging.IBitmapProperties";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("ea9f4f1b-b505-4450-a4d1-e8ca94529d8d")
                IBitmapProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetPropertiesAsync(
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* propertiesToSet,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapProperties = __uuidof(IBitmapProperties);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapPropertiesView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapPropertiesView[] = L"Windows.Graphics.Imaging.IBitmapPropertiesView";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("7e0fe87a-3a70-48f8-9c55-196cf5a545f5")
                IBitmapPropertiesView : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPropertiesAsync(
                        __FIIterable_1_HSTRING* propertiesToRetrieve,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapPropertiesView = __uuidof(IBitmapPropertiesView);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapTransform
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapTransform[] = L"Windows.Graphics.Imaging.IBitmapTransform";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("ae755344-e268-4d35-adcf-e995d31a8d34")
                IBitmapTransform : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ScaledWidth(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ScaledWidth(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScaledHeight(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ScaledHeight(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InterpolationMode(
                        ABI::Windows::Graphics::Imaging::BitmapInterpolationMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InterpolationMode(
                        ABI::Windows::Graphics::Imaging::BitmapInterpolationMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Flip(
                        ABI::Windows::Graphics::Imaging::BitmapFlip* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Flip(
                        ABI::Windows::Graphics::Imaging::BitmapFlip value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rotation(
                        ABI::Windows::Graphics::Imaging::BitmapRotation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Rotation(
                        ABI::Windows::Graphics::Imaging::BitmapRotation value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bounds(
                        ABI::Windows::Graphics::Imaging::BitmapBounds* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Bounds(
                        ABI::Windows::Graphics::Imaging::BitmapBounds value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapTransform = __uuidof(IBitmapTransform);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapTypedValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapTypedValue
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapTypedValue[] = L"Windows.Graphics.Imaging.IBitmapTypedValue";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("cd8044a9-2443-4000-b0cd-79316c56f589")
                IBitmapTypedValue : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Foundation::PropertyType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapTypedValue = __uuidof(IBitmapTypedValue);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapTypedValueFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapTypedValue
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapTypedValueFactory[] = L"Windows.Graphics.Imaging.IBitmapTypedValueFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("92dbb599-ce13-46bb-9545-cb3a3f63eb8b")
                IBitmapTypedValueFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        IInspectable* value,
                        ABI::Windows::Foundation::PropertyType type,
                        ABI::Windows::Graphics::Imaging::IBitmapTypedValue** bitmapTypedValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBitmapTypedValueFactory = __uuidof(IBitmapTypedValueFactory);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IPixelDataProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.PixelDataProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IPixelDataProvider[] = L"Windows.Graphics.Imaging.IPixelDataProvider";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("dd831f25-185c-4595-9fb9-ccbe6ec18a6f")
                IPixelDataProvider : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE DetachPixelData(
                        UINT32* pixelDataLength,
                        BYTE** pixelData
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPixelDataProvider = __uuidof(IPixelDataProvider);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.ISoftwareBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.SoftwareBitmap
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_ISoftwareBitmap[] = L"Windows.Graphics.Imaging.ISoftwareBitmap";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("689e0708-7eef-483f-963f-da938818e073")
                ISoftwareBitmap : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapPixelFormat(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitmapAlphaMode(
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PixelWidth(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PixelHeight(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsReadOnly(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DpiX(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DpiX(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DpiY(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DpiY(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LockBuffer(
                        ABI::Windows::Graphics::Imaging::BitmapBufferAccessMode mode,
                        ABI::Windows::Graphics::Imaging::IBitmapBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CopyTo(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap* bitmap
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CopyFromBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CopyToBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetReadOnlyView(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISoftwareBitmap = __uuidof(ISoftwareBitmap);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.ISoftwareBitmapFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.SoftwareBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_ISoftwareBitmapFactory[] = L"Windows.Graphics.Imaging.ISoftwareBitmapFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("c99feb69-2d62-4d47-a6b3-4fdb6a07fdf8")
                ISoftwareBitmapFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                        INT32 width,
                        INT32 height,
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAlpha(
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                        INT32 width,
                        INT32 height,
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode alpha,
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISoftwareBitmapFactory = __uuidof(ISoftwareBitmapFactory);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.ISoftwareBitmapStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.SoftwareBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_ISoftwareBitmapStatics[] = L"Windows.Graphics.Imaging.ISoftwareBitmapStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                MIDL_INTERFACE("df0385db-672f-4a9d-806e-c2442f343e86")
                ISoftwareBitmapStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Copy(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap* source,
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Convert(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap* source,
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConvertWithAlpha(
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap* source,
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode alpha,
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCopyFromBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* source,
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                        INT32 width,
                        INT32 height,
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCopyWithAlphaFromBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* source,
                        ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                        INT32 width,
                        INT32 height,
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode alpha,
                        ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCopyFromSurfaceAsync(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface* surface,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCopyWithAlphaFromSurfaceAsync(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface* surface,
                        ABI::Windows::Graphics::Imaging::BitmapAlphaMode alpha,
                        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISoftwareBitmapStatics = __uuidof(ISoftwareBitmapStatics);
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapBuffer ** Default Interface **
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapBuffer_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapBuffer[] = L"Windows.Graphics.Imaging.BitmapBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapCodecInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapCodecInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapCodecInformation_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapCodecInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapCodecInformation[] = L"Windows.Graphics.Imaging.BitmapCodecInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapDecoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Imaging.IBitmapDecoderStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Imaging.IBitmapDecoderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapDecoder ** Default Interface **
 *    Windows.Graphics.Imaging.IBitmapFrame
 *    Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapDecoder_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapDecoder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapDecoder[] = L"Windows.Graphics.Imaging.BitmapDecoder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapEncoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Imaging.IBitmapEncoderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Imaging.IBitmapEncoderStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapEncoder ** Default Interface **
 *    Windows.Graphics.Imaging.IBitmapEncoderWithSoftwareBitmap
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapEncoder_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapEncoder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapEncoder[] = L"Windows.Graphics.Imaging.BitmapEncoder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapFrame ** Default Interface **
 *    Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapFrame_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapFrame[] = L"Windows.Graphics.Imaging.BitmapFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapProperties ** Default Interface **
 *    Windows.Graphics.Imaging.IBitmapPropertiesView
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapProperties_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapProperties[] = L"Windows.Graphics.Imaging.BitmapProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapPropertiesView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapPropertiesView ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapPropertiesView_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapPropertiesView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapPropertiesView[] = L"Windows.Graphics.Imaging.BitmapPropertiesView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapPropertySet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMap`2<String, Windows.Graphics.Imaging.BitmapTypedValue> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Graphics.Imaging.BitmapTypedValue>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapPropertySet_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapPropertySet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapPropertySet[] = L"Windows.Graphics.Imaging.BitmapPropertySet";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapTransform ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapTransform_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapTransform_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapTransform[] = L"Windows.Graphics.Imaging.BitmapTransform";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapTypedValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Imaging.IBitmapTypedValueFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapTypedValue ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapTypedValue_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapTypedValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapTypedValue[] = L"Windows.Graphics.Imaging.BitmapTypedValue";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.ImageStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamWithContentType ** Default Interface **
 *    Windows.Storage.Streams.IContentTypeProvider
 *    Windows.Storage.Streams.IRandomAccessStream
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_ImageStream_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_ImageStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_ImageStream[] = L"Windows.Graphics.Imaging.ImageStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.PixelDataProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IPixelDataProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_PixelDataProvider_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_PixelDataProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_PixelDataProvider[] = L"Windows.Graphics.Imaging.PixelDataProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.SoftwareBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Imaging.ISoftwareBitmapFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Imaging.ISoftwareBitmapStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.ISoftwareBitmap ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_SoftwareBitmap_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_SoftwareBitmap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_SoftwareBitmap[] = L"Windows.Graphics.Imaging.SoftwareBitmap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2 __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2 __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoderVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapDecoder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoderVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapEncoder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrameVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrameVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapFrame_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

typedef struct __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING key,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue** first,
        __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl;

interface __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue;

typedef struct __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING key,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        __FIMapView_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING key,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* This);

    END_INTERFACE
} __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl;

interface __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue
{
    CONST_VTBL struct __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        __FIMap_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySetVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySetVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CBitmapPropertySet_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProviderVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProviderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CPixelDataProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmapVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmapVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CImaging__CSoftwareBitmap_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation;

typedef struct __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl;

interface __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation;

typedef struct __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        __FIIterator_1_Windows__CGraphics__CImaging__CBitmapCodecInformation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl;

interface __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation;

typedef struct __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl;

interface __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBuffer __x_ABI_CWindows_CFoundation_CIMemoryBuffer;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CFoundation_CPropertyType __x_ABI_CWindows_CFoundation_CPropertyType;

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

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

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapBufferAccessMode __x_ABI_CWindows_CGraphics_CImaging_CBitmapBufferAccessMode;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapFlip __x_ABI_CWindows_CGraphics_CImaging_CBitmapFlip;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapInterpolationMode __x_ABI_CWindows_CGraphics_CImaging_CBitmapInterpolationMode;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapRotation __x_ABI_CWindows_CGraphics_CImaging_CBitmapRotation;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CColorManagementMode __x_ABI_CWindows_CGraphics_CImaging_CColorManagementMode;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CExifOrientationMode __x_ABI_CWindows_CGraphics_CImaging_CExifOrientationMode;

typedef struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds;

typedef struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapPlaneDescription __x_ABI_CWindows_CGraphics_CImaging_CBitmapPlaneDescription;

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapAlphaMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode
{
    BitmapAlphaMode_Premultiplied = 0,
    BitmapAlphaMode_Straight = 1,
    BitmapAlphaMode_Ignore = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapBufferAccessMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapBufferAccessMode
{
    BitmapBufferAccessMode_Read = 0,
    BitmapBufferAccessMode_ReadWrite = 1,
    BitmapBufferAccessMode_Write = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapFlip
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapFlip
{
    BitmapFlip_None = 0,
    BitmapFlip_Horizontal = 1,
    BitmapFlip_Vertical = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapInterpolationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapInterpolationMode
{
    BitmapInterpolationMode_NearestNeighbor = 0,
    BitmapInterpolationMode_Linear = 1,
    BitmapInterpolationMode_Cubic = 2,
    BitmapInterpolationMode_Fant = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapPixelFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat
{
    BitmapPixelFormat_Unknown = 0,
    BitmapPixelFormat_Rgba16 = 12,
    BitmapPixelFormat_Rgba8 = 30,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    BitmapPixelFormat_Gray16 = 57,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    BitmapPixelFormat_Gray8 = 62,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    BitmapPixelFormat_Bgra8 = 87,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    BitmapPixelFormat_Nv12 = 103,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    BitmapPixelFormat_P010 = 104,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    BitmapPixelFormat_Yuy2 = 107,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapRotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapRotation
{
    BitmapRotation_None = 0,
    BitmapRotation_Clockwise90Degrees = 1,
    BitmapRotation_Clockwise180Degrees = 2,
    BitmapRotation_Clockwise270Degrees = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.ColorManagementMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CColorManagementMode
{
    ColorManagementMode_DoNotColorManage = 0,
    ColorManagementMode_ColorManageToSRgb = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.ExifOrientationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CExifOrientationMode
{
    ExifOrientationMode_IgnoreExifOrientation = 0,
    ExifOrientationMode_RespectExifOrientation = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.JpegSubsamplingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CJpegSubsamplingMode
{
    JpegSubsamplingMode_Default = 0,
    JpegSubsamplingMode_Y4Cb2Cr0 = 1,
    JpegSubsamplingMode_Y4Cb2Cr2 = 2,
    JpegSubsamplingMode_Y4Cb4Cr4 = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.PngFilterMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CPngFilterMode
{
    PngFilterMode_Automatic = 0,
    PngFilterMode_None = 1,
    PngFilterMode_Sub = 2,
    PngFilterMode_Up = 3,
    PngFilterMode_Average = 4,
    PngFilterMode_Paeth = 5,
    PngFilterMode_Adaptive = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.TiffCompressionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CImaging_CTiffCompressionMode
{
    TiffCompressionMode_Automatic = 0,
    TiffCompressionMode_None = 1,
    TiffCompressionMode_Ccitt3 = 2,
    TiffCompressionMode_Ccitt4 = 3,
    TiffCompressionMode_Lzw = 4,
    TiffCompressionMode_Rle = 5,
    TiffCompressionMode_Zip = 6,
    TiffCompressionMode_LzwhDifferencing = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds
{
    UINT32 X;
    UINT32 Y;
    UINT32 Width;
    UINT32 Height;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapPlaneDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapPlaneDescription
{
    INT32 StartIndex;
    INT32 Width;
    INT32 Height;
    INT32 Stride;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Imaging.BitmapSize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize
{
    UINT32 Width;
    UINT32 Height;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapBuffer
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IMemoryBuffer
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapBuffer[] = L"Windows.Graphics.Imaging.IBitmapBuffer";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPlaneCount)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* GetPlaneDescription)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer* This,
        INT32 index,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapPlaneDescription* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBufferVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_GetPlaneCount(This, value) \
    ((This)->lpVtbl->GetPlaneCount(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_GetPlaneDescription(This, index, value) \
    ((This)->lpVtbl->GetPlaneDescription(This, index, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapCodecInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapCodecInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapCodecInformation[] = L"Windows.Graphics.Imaging.IBitmapCodecInformation";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CodecId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_FileExtensions)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MimeTypes)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformationVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_get_CodecId(This, value) \
    ((This)->lpVtbl->get_CodecId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_get_FileExtensions(This, value) \
    ((This)->lpVtbl->get_FileExtensions(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_get_MimeTypes(This, value) \
    ((This)->lpVtbl->get_MimeTypes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapDecoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapDecoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapDecoder[] = L"Windows.Graphics.Imaging.IBitmapDecoder";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BitmapContainerProperties)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView** value);
    HRESULT (STDMETHODCALLTYPE* get_DecoderInformation)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_FrameCount)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* GetPreviewAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* GetFrameAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* This,
        UINT32 frameIndex,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapFrame** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_get_BitmapContainerProperties(This, value) \
    ((This)->lpVtbl->get_BitmapContainerProperties(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_get_DecoderInformation(This, value) \
    ((This)->lpVtbl->get_DecoderInformation(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_get_FrameCount(This, value) \
    ((This)->lpVtbl->get_FrameCount(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_GetPreviewAsync(This, asyncInfo) \
    ((This)->lpVtbl->GetPreviewAsync(This, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_GetFrameAsync(This, frameIndex, asyncInfo) \
    ((This)->lpVtbl->GetFrameAsync(This, frameIndex, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapDecoderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapDecoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapDecoderStatics[] = L"Windows.Graphics.Imaging.IBitmapDecoderStatics";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BmpDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_JpegDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_PngDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_TiffDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_GifDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_JpegXRDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_IcoDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* GetDecoderInformationEnumerator)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation** decoderInformationEnumerator);
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* stream,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* CreateWithIdAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics* This,
        GUID decoderId,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* stream,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapDecoder** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_get_BmpDecoderId(This, value) \
    ((This)->lpVtbl->get_BmpDecoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_get_JpegDecoderId(This, value) \
    ((This)->lpVtbl->get_JpegDecoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_get_PngDecoderId(This, value) \
    ((This)->lpVtbl->get_PngDecoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_get_TiffDecoderId(This, value) \
    ((This)->lpVtbl->get_TiffDecoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_get_GifDecoderId(This, value) \
    ((This)->lpVtbl->get_GifDecoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_get_JpegXRDecoderId(This, value) \
    ((This)->lpVtbl->get_JpegXRDecoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_get_IcoDecoderId(This, value) \
    ((This)->lpVtbl->get_IcoDecoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_GetDecoderInformationEnumerator(This, decoderInformationEnumerator) \
    ((This)->lpVtbl->GetDecoderInformationEnumerator(This, decoderInformationEnumerator))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_CreateAsync(This, stream, asyncInfo) \
    ((This)->lpVtbl->CreateAsync(This, stream, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_CreateWithIdAsync(This, decoderId, stream, asyncInfo) \
    ((This)->lpVtbl->CreateWithIdAsync(This, decoderId, stream, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapDecoderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapDecoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapDecoderStatics2[] = L"Windows.Graphics.Imaging.IBitmapDecoderStatics2";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HeifDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_WebpDecoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2Vtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_get_HeifDecoderId(This, value) \
    ((This)->lpVtbl->get_HeifDecoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_get_WebpDecoderId(This, value) \
    ((This)->lpVtbl->get_WebpDecoderId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapEncoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapEncoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapEncoder[] = L"Windows.Graphics.Imaging.IBitmapEncoder";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EncoderInformation)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapCodecInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_BitmapProperties)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_BitmapContainerProperties)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_IsThumbnailGenerated)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsThumbnailGenerated)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_GeneratedThumbnailWidth)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_GeneratedThumbnailWidth)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_GeneratedThumbnailHeight)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_GeneratedThumbnailHeight)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BitmapTransform)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform** value);
    HRESULT (STDMETHODCALLTYPE* SetPixelData)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat pixelFormat,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alphaMode,
        UINT32 width,
        UINT32 height,
        DOUBLE dpiX,
        DOUBLE dpiY,
        UINT32 pixelsLength,
        BYTE* pixels);
    HRESULT (STDMETHODCALLTYPE* GoToNextFrameAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* GoToNextFrameWithEncodingOptionsAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* encodingOptions,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* FlushAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_get_EncoderInformation(This, value) \
    ((This)->lpVtbl->get_EncoderInformation(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_get_BitmapProperties(This, value) \
    ((This)->lpVtbl->get_BitmapProperties(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_get_BitmapContainerProperties(This, value) \
    ((This)->lpVtbl->get_BitmapContainerProperties(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_get_IsThumbnailGenerated(This, value) \
    ((This)->lpVtbl->get_IsThumbnailGenerated(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_put_IsThumbnailGenerated(This, value) \
    ((This)->lpVtbl->put_IsThumbnailGenerated(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_get_GeneratedThumbnailWidth(This, value) \
    ((This)->lpVtbl->get_GeneratedThumbnailWidth(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_put_GeneratedThumbnailWidth(This, value) \
    ((This)->lpVtbl->put_GeneratedThumbnailWidth(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_get_GeneratedThumbnailHeight(This, value) \
    ((This)->lpVtbl->get_GeneratedThumbnailHeight(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_put_GeneratedThumbnailHeight(This, value) \
    ((This)->lpVtbl->put_GeneratedThumbnailHeight(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_get_BitmapTransform(This, value) \
    ((This)->lpVtbl->get_BitmapTransform(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_SetPixelData(This, pixelFormat, alphaMode, width, height, dpiX, dpiY, pixelsLength, pixels) \
    ((This)->lpVtbl->SetPixelData(This, pixelFormat, alphaMode, width, height, dpiX, dpiY, pixelsLength, pixels))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_GoToNextFrameAsync(This, asyncInfo) \
    ((This)->lpVtbl->GoToNextFrameAsync(This, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_GoToNextFrameWithEncodingOptionsAsync(This, encodingOptions, asyncInfo) \
    ((This)->lpVtbl->GoToNextFrameWithEncodingOptionsAsync(This, encodingOptions, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_FlushAsync(This, asyncInfo) \
    ((This)->lpVtbl->FlushAsync(This, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapEncoderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapEncoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapEncoderStatics[] = L"Windows.Graphics.Imaging.IBitmapEncoderStatics";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BmpEncoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_JpegEncoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_PngEncoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_TiffEncoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_GifEncoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_JpegXREncoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* GetEncoderInformationEnumerator)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        __FIVectorView_1_Windows__CGraphics__CImaging__CBitmapCodecInformation** encoderInformationEnumerator);
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        GUID encoderId,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* stream,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* CreateWithEncodingOptionsAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        GUID encoderId,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* stream,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* encodingOptions,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* CreateForTranscodingAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* stream,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* bitmapDecoder,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* CreateForInPlacePropertyEncodingAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapDecoder* bitmapDecoder,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapEncoder** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_get_BmpEncoderId(This, value) \
    ((This)->lpVtbl->get_BmpEncoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_get_JpegEncoderId(This, value) \
    ((This)->lpVtbl->get_JpegEncoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_get_PngEncoderId(This, value) \
    ((This)->lpVtbl->get_PngEncoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_get_TiffEncoderId(This, value) \
    ((This)->lpVtbl->get_TiffEncoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_get_GifEncoderId(This, value) \
    ((This)->lpVtbl->get_GifEncoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_get_JpegXREncoderId(This, value) \
    ((This)->lpVtbl->get_JpegXREncoderId(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_GetEncoderInformationEnumerator(This, encoderInformationEnumerator) \
    ((This)->lpVtbl->GetEncoderInformationEnumerator(This, encoderInformationEnumerator))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_CreateAsync(This, encoderId, stream, asyncInfo) \
    ((This)->lpVtbl->CreateAsync(This, encoderId, stream, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_CreateWithEncodingOptionsAsync(This, encoderId, stream, encodingOptions, asyncInfo) \
    ((This)->lpVtbl->CreateWithEncodingOptionsAsync(This, encoderId, stream, encodingOptions, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_CreateForTranscodingAsync(This, stream, bitmapDecoder, asyncInfo) \
    ((This)->lpVtbl->CreateForTranscodingAsync(This, stream, bitmapDecoder, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_CreateForInPlacePropertyEncodingAsync(This, bitmapDecoder, asyncInfo) \
    ((This)->lpVtbl->CreateForInPlacePropertyEncodingAsync(This, bitmapDecoder, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapEncoderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapEncoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapEncoderStatics2[] = L"Windows.Graphics.Imaging.IBitmapEncoderStatics2";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HeifEncoderId)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2Vtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_get_HeifEncoderId(This, value) \
    ((This)->lpVtbl->get_HeifEncoderId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapEncoderWithSoftwareBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapEncoder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapEncoderWithSoftwareBitmap[] = L"Windows.Graphics.Imaging.IBitmapEncoderWithSoftwareBitmap";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetSoftwareBitmap)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* bitmap);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmapVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_SetSoftwareBitmap(This, bitmap) \
    ((This)->lpVtbl->SetSoftwareBitmap(This, bitmap))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapEncoderWithSoftwareBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapFrame[] = L"Windows.Graphics.Imaging.IBitmapFrame";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetThumbnailAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CImageStream** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* get_BitmapProperties)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView** value);
    HRESULT (STDMETHODCALLTYPE* get_BitmapPixelFormat)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_BitmapAlphaMode)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode* value);
    HRESULT (STDMETHODCALLTYPE* get_DpiX)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_DpiY)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_PixelWidth)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PixelHeight)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_OrientedPixelWidth)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_OrientedPixelHeight)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* GetPixelDataAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* GetPixelDataTransformedAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat pixelFormat,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alphaMode,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* transform,
        enum __x_ABI_CWindows_CGraphics_CImaging_CExifOrientationMode exifOrientationMode,
        enum __x_ABI_CWindows_CGraphics_CImaging_CColorManagementMode colorManagementMode,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CPixelDataProvider** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_GetThumbnailAsync(This, asyncInfo) \
    ((This)->lpVtbl->GetThumbnailAsync(This, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_BitmapProperties(This, value) \
    ((This)->lpVtbl->get_BitmapProperties(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_BitmapPixelFormat(This, value) \
    ((This)->lpVtbl->get_BitmapPixelFormat(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_BitmapAlphaMode(This, value) \
    ((This)->lpVtbl->get_BitmapAlphaMode(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_DpiX(This, value) \
    ((This)->lpVtbl->get_DpiX(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_DpiY(This, value) \
    ((This)->lpVtbl->get_DpiY(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_PixelWidth(This, value) \
    ((This)->lpVtbl->get_PixelWidth(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_PixelHeight(This, value) \
    ((This)->lpVtbl->get_PixelHeight(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_OrientedPixelWidth(This, value) \
    ((This)->lpVtbl->get_OrientedPixelWidth(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_get_OrientedPixelHeight(This, value) \
    ((This)->lpVtbl->get_OrientedPixelHeight(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_GetPixelDataAsync(This, asyncInfo) \
    ((This)->lpVtbl->GetPixelDataAsync(This, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_GetPixelDataTransformedAsync(This, pixelFormat, alphaMode, transform, exifOrientationMode, colorManagementMode, asyncInfo) \
    ((This)->lpVtbl->GetPixelDataTransformedAsync(This, pixelFormat, alphaMode, transform, exifOrientationMode, colorManagementMode, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Imaging.IBitmapFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapFrameWithSoftwareBitmap[] = L"Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetSoftwareBitmapAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* GetSoftwareBitmapConvertedAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat pixelFormat,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alphaMode,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* GetSoftwareBitmapTransformedAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat pixelFormat,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alphaMode,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* transform,
        enum __x_ABI_CWindows_CGraphics_CImaging_CExifOrientationMode exifOrientationMode,
        enum __x_ABI_CWindows_CGraphics_CImaging_CColorManagementMode colorManagementMode,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmapVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_GetSoftwareBitmapAsync(This, value) \
    ((This)->lpVtbl->GetSoftwareBitmapAsync(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_GetSoftwareBitmapConvertedAsync(This, pixelFormat, alphaMode, value) \
    ((This)->lpVtbl->GetSoftwareBitmapConvertedAsync(This, pixelFormat, alphaMode, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_GetSoftwareBitmapTransformedAsync(This, pixelFormat, alphaMode, transform, exifOrientationMode, colorManagementMode, value) \
    ((This)->lpVtbl->GetSoftwareBitmapTransformedAsync(This, pixelFormat, alphaMode, transform, exifOrientationMode, colorManagementMode, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapFrameWithSoftwareBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Imaging.IBitmapPropertiesView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapProperties[] = L"Windows.Graphics.Imaging.IBitmapProperties";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetPropertiesAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties* This,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CImaging__CBitmapTypedValue* propertiesToSet,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_SetPropertiesAsync(This, propertiesToSet, asyncInfo) \
    ((This)->lpVtbl->SetPropertiesAsync(This, propertiesToSet, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapPropertiesView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapPropertiesView[] = L"Windows.Graphics.Imaging.IBitmapPropertiesView";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPropertiesAsync)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView* This,
        __FIIterable_1_HSTRING* propertiesToRetrieve,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CBitmapPropertySet** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesViewVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_GetPropertiesAsync(This, propertiesToRetrieve, asyncInfo) \
    ((This)->lpVtbl->GetPropertiesAsync(This, propertiesToRetrieve, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapPropertiesView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapTransform
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapTransform[] = L"Windows.Graphics.Imaging.IBitmapTransform";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransformVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ScaledWidth)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ScaledWidth)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ScaledHeight)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ScaledHeight)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_InterpolationMode)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapInterpolationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_InterpolationMode)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapInterpolationMode value);
    HRESULT (STDMETHODCALLTYPE* get_Flip)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapFlip* value);
    HRESULT (STDMETHODCALLTYPE* put_Flip)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapFlip value);
    HRESULT (STDMETHODCALLTYPE* get_Rotation)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapRotation* value);
    HRESULT (STDMETHODCALLTYPE* put_Rotation)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapRotation value);
    HRESULT (STDMETHODCALLTYPE* get_Bounds)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds* value);
    HRESULT (STDMETHODCALLTYPE* put_Bounds)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransformVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransformVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_get_ScaledWidth(This, value) \
    ((This)->lpVtbl->get_ScaledWidth(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_put_ScaledWidth(This, value) \
    ((This)->lpVtbl->put_ScaledWidth(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_get_ScaledHeight(This, value) \
    ((This)->lpVtbl->get_ScaledHeight(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_put_ScaledHeight(This, value) \
    ((This)->lpVtbl->put_ScaledHeight(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_get_InterpolationMode(This, value) \
    ((This)->lpVtbl->get_InterpolationMode(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_put_InterpolationMode(This, value) \
    ((This)->lpVtbl->put_InterpolationMode(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_get_Flip(This, value) \
    ((This)->lpVtbl->get_Flip(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_put_Flip(This, value) \
    ((This)->lpVtbl->put_Flip(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_get_Rotation(This, value) \
    ((This)->lpVtbl->get_Rotation(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_put_Rotation(This, value) \
    ((This)->lpVtbl->put_Rotation(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_get_Bounds(This, value) \
    ((This)->lpVtbl->get_Bounds(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_put_Bounds(This, value) \
    ((This)->lpVtbl->put_Bounds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTransform_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapTypedValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapTypedValue
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapTypedValue[] = L"Windows.Graphics.Imaging.IBitmapTypedValue";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue* This,
        enum __x_ABI_CWindows_CFoundation_CPropertyType* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IBitmapTypedValueFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.BitmapTypedValue
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IBitmapTypedValueFactory[] = L"Windows.Graphics.Imaging.IBitmapTypedValueFactory";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory* This,
        IInspectable* value,
        enum __x_ABI_CWindows_CFoundation_CPropertyType type,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValue** bitmapTypedValue);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_Create(This, value, type, bitmapTypedValue) \
    ((This)->lpVtbl->Create(This, value, type, bitmapTypedValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIBitmapTypedValueFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.IPixelDataProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.PixelDataProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_IPixelDataProvider[] = L"Windows.Graphics.Imaging.IPixelDataProvider";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DetachPixelData)(__x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider* This,
        UINT32* pixelDataLength,
        BYTE** pixelData);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProviderVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_DetachPixelData(This, pixelDataLength, pixelData) \
    ((This)->lpVtbl->DetachPixelData(This, pixelDataLength, pixelData))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CIPixelDataProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.ISoftwareBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.SoftwareBitmap
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_ISoftwareBitmap[] = L"Windows.Graphics.Imaging.ISoftwareBitmap";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BitmapPixelFormat)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_BitmapAlphaMode)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode* value);
    HRESULT (STDMETHODCALLTYPE* get_PixelWidth)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PixelHeight)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnly)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_DpiX)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_DpiX)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_DpiY)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_DpiY)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* LockBuffer)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapBufferAccessMode mode,
        __x_ABI_CWindows_CGraphics_CImaging_CIBitmapBuffer** value);
    HRESULT (STDMETHODCALLTYPE* CopyTo)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* bitmap);
    HRESULT (STDMETHODCALLTYPE* CopyFromBuffer)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer);
    HRESULT (STDMETHODCALLTYPE* CopyToBuffer)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer);
    HRESULT (STDMETHODCALLTYPE* GetReadOnlyView)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_get_BitmapPixelFormat(This, value) \
    ((This)->lpVtbl->get_BitmapPixelFormat(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_get_BitmapAlphaMode(This, value) \
    ((This)->lpVtbl->get_BitmapAlphaMode(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_get_PixelWidth(This, value) \
    ((This)->lpVtbl->get_PixelWidth(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_get_PixelHeight(This, value) \
    ((This)->lpVtbl->get_PixelHeight(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_get_IsReadOnly(This, value) \
    ((This)->lpVtbl->get_IsReadOnly(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_put_DpiX(This, value) \
    ((This)->lpVtbl->put_DpiX(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_get_DpiX(This, value) \
    ((This)->lpVtbl->get_DpiX(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_put_DpiY(This, value) \
    ((This)->lpVtbl->put_DpiY(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_get_DpiY(This, value) \
    ((This)->lpVtbl->get_DpiY(This, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_LockBuffer(This, mode, value) \
    ((This)->lpVtbl->LockBuffer(This, mode, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_CopyTo(This, bitmap) \
    ((This)->lpVtbl->CopyTo(This, bitmap))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_CopyFromBuffer(This, buffer) \
    ((This)->lpVtbl->CopyFromBuffer(This, buffer))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_CopyToBuffer(This, buffer) \
    ((This)->lpVtbl->CopyToBuffer(This, buffer))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_GetReadOnlyView(This, value) \
    ((This)->lpVtbl->GetReadOnlyView(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.ISoftwareBitmapFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.SoftwareBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_ISoftwareBitmapFactory[] = L"Windows.Graphics.Imaging.ISoftwareBitmapFactory";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        INT32 width,
        INT32 height,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAlpha)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        INT32 width,
        INT32 height,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alpha,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_Create(This, format, width, height, value) \
    ((This)->lpVtbl->Create(This, format, width, height, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_CreateWithAlpha(This, format, width, height, alpha, value) \
    ((This)->lpVtbl->CreateWithAlpha(This, format, width, height, alpha, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Imaging.ISoftwareBitmapStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Imaging.SoftwareBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Imaging_ISoftwareBitmapStatics[] = L"Windows.Graphics.Imaging.ISoftwareBitmapStatics";
typedef struct __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Copy)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* source,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* Convert)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* source,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* ConvertWithAlpha)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* source,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alpha,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* CreateCopyFromBuffer)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* source,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        INT32 width,
        INT32 height,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* CreateCopyWithAlphaFromBuffer)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* source,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        INT32 width,
        INT32 height,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alpha,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* CreateCopyFromSurfaceAsync)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* surface,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* CreateCopyWithAlphaFromSurfaceAsync)(__x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* surface,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alpha,
        __FIAsyncOperation_1_Windows__CGraphics__CImaging__CSoftwareBitmap** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_Copy(This, source, value) \
    ((This)->lpVtbl->Copy(This, source, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_Convert(This, source, format, value) \
    ((This)->lpVtbl->Convert(This, source, format, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_ConvertWithAlpha(This, source, format, alpha, value) \
    ((This)->lpVtbl->ConvertWithAlpha(This, source, format, alpha, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_CreateCopyFromBuffer(This, source, format, width, height, value) \
    ((This)->lpVtbl->CreateCopyFromBuffer(This, source, format, width, height, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_CreateCopyWithAlphaFromBuffer(This, source, format, width, height, alpha, value) \
    ((This)->lpVtbl->CreateCopyWithAlphaFromBuffer(This, source, format, width, height, alpha, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_CreateCopyFromSurfaceAsync(This, surface, value) \
    ((This)->lpVtbl->CreateCopyFromSurfaceAsync(This, surface, value))

#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_CreateCopyWithAlphaFromSurfaceAsync(This, surface, alpha, value) \
    ((This)->lpVtbl->CreateCopyWithAlphaFromSurfaceAsync(This, surface, alpha, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmapStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapBuffer ** Default Interface **
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapBuffer_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapBuffer[] = L"Windows.Graphics.Imaging.BitmapBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapCodecInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapCodecInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapCodecInformation_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapCodecInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapCodecInformation[] = L"Windows.Graphics.Imaging.BitmapCodecInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapDecoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Imaging.IBitmapDecoderStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Imaging.IBitmapDecoderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapDecoder ** Default Interface **
 *    Windows.Graphics.Imaging.IBitmapFrame
 *    Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapDecoder_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapDecoder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapDecoder[] = L"Windows.Graphics.Imaging.BitmapDecoder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapEncoder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Imaging.IBitmapEncoderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Imaging.IBitmapEncoderStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapEncoder ** Default Interface **
 *    Windows.Graphics.Imaging.IBitmapEncoderWithSoftwareBitmap
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapEncoder_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapEncoder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapEncoder[] = L"Windows.Graphics.Imaging.BitmapEncoder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapFrame ** Default Interface **
 *    Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapFrame_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapFrame[] = L"Windows.Graphics.Imaging.BitmapFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapProperties ** Default Interface **
 *    Windows.Graphics.Imaging.IBitmapPropertiesView
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapProperties_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapProperties[] = L"Windows.Graphics.Imaging.BitmapProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapPropertiesView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapPropertiesView ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapPropertiesView_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapPropertiesView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapPropertiesView[] = L"Windows.Graphics.Imaging.BitmapPropertiesView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapPropertySet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMap`2<String, Windows.Graphics.Imaging.BitmapTypedValue> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Graphics.Imaging.BitmapTypedValue>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapPropertySet_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapPropertySet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapPropertySet[] = L"Windows.Graphics.Imaging.BitmapPropertySet";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapTransform ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapTransform_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapTransform_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapTransform[] = L"Windows.Graphics.Imaging.BitmapTransform";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.BitmapTypedValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Imaging.IBitmapTypedValueFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IBitmapTypedValue ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_BitmapTypedValue_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_BitmapTypedValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_BitmapTypedValue[] = L"Windows.Graphics.Imaging.BitmapTypedValue";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.ImageStream
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamWithContentType ** Default Interface **
 *    Windows.Storage.Streams.IContentTypeProvider
 *    Windows.Storage.Streams.IRandomAccessStream
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_ImageStream_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_ImageStream_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_ImageStream[] = L"Windows.Graphics.Imaging.ImageStream";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.PixelDataProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.IPixelDataProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_PixelDataProvider_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_PixelDataProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_PixelDataProvider[] = L"Windows.Graphics.Imaging.PixelDataProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Imaging.SoftwareBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Imaging.ISoftwareBitmapFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Imaging.ISoftwareBitmapStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Imaging.ISoftwareBitmap ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Imaging_SoftwareBitmap_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Imaging_SoftwareBitmap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Imaging_SoftwareBitmap[] = L"Windows.Graphics.Imaging.SoftwareBitmap";
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
#endif // __windows2Egraphics2Eimaging_p_h__

#endif // __windows2Egraphics2Eimaging_h__
