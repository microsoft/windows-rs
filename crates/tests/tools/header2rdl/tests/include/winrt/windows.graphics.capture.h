
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
#ifndef __windows2Egraphics2Ecapture_h__
#define __windows2Egraphics2Ecapture_h__
#ifndef __windows2Egraphics2Ecapture_p_h__
#define __windows2Egraphics2Ecapture_p_h__


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

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Graphics.h"
#include "Windows.Graphics.DirectX.h"
#include "Windows.Graphics.DirectX.Direct3D11.h"
#include "Windows.Security.Authorization.AppCapabilityAccess.h"
#include "Windows.System.h"
#include "Windows.UI.h"
#include "Windows.UI.Composition.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IDirect3D11CaptureFrame;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame ABI::Windows::Graphics::Capture::IDirect3D11CaptureFrame

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IDirect3D11CaptureFrame2;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2 ABI::Windows::Graphics::Capture::IDirect3D11CaptureFrame2

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IDirect3D11CaptureFramePool;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool ABI::Windows::Graphics::Capture::IDirect3D11CaptureFramePool

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IDirect3D11CaptureFramePoolStatics;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics ABI::Windows::Graphics::Capture::IDirect3D11CaptureFramePoolStatics

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IDirect3D11CaptureFramePoolStatics2;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2 ABI::Windows::Graphics::Capture::IDirect3D11CaptureFramePoolStatics2

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureAccessStatics;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics ABI::Windows::Graphics::Capture::IGraphicsCaptureAccessStatics

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureItem;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem ABI::Windows::Graphics::Capture::IGraphicsCaptureItem

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureItemStatics;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics ABI::Windows::Graphics::Capture::IGraphicsCaptureItemStatics

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureItemStatics2;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2 ABI::Windows::Graphics::Capture::IGraphicsCaptureItemStatics2

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCapturePicker;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker ABI::Windows::Graphics::Capture::IGraphicsCapturePicker

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureSession;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession ABI::Windows::Graphics::Capture::IGraphicsCaptureSession

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureSession2;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2 ABI::Windows::Graphics::Capture::IGraphicsCaptureSession2

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureSession3;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3 ABI::Windows::Graphics::Capture::IGraphicsCaptureSession3

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureSession4;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4 ABI::Windows::Graphics::Capture::IGraphicsCaptureSession4

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureSession5;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5 ABI::Windows::Graphics::Capture::IGraphicsCaptureSession5

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureSession6;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6 ABI::Windows::Graphics::Capture::IGraphicsCaptureSession6

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                interface IGraphicsCaptureSessionStatics;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics ABI::Windows::Graphics::Capture::IGraphicsCaptureSessionStatics

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                class GraphicsCaptureItem;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("01ccf2ae-1059-5d57-a805-0a1dfc54cab9"))
IAsyncOperation<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*, ABI::Windows::Graphics::Capture::IGraphicsCaptureItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Capture.GraphicsCaptureItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*> __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("43004a3c-ffe7-5352-85a6-7bc41b782fca"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*, ABI::Windows::Graphics::Capture::IGraphicsCaptureItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Capture.GraphicsCaptureItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    typedef enum AppCapabilityAccessStatus : int AppCapabilityAccessStatus;
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("827caf42-5fe6-5b5b-84ce-c44834134d3d"))
IAsyncOperation<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6ea0f2e9-bc97-58e8-a3a6-c829b9e5f2aa"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct RectInt32 RectInt32;
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CRectInt32_USE
#define DEF___FIIterator_1_Windows__CGraphics__CRectInt32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1abdf3f6-23f1-55ad-babd-f4cd908406e7"))
IIterator<struct ABI::Windows::Graphics::RectInt32> : IIterator_impl<struct ABI::Windows::Graphics::RectInt32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.RectInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Graphics::RectInt32> __FIIterator_1_Windows__CGraphics__CRectInt32_t;
#define __FIIterator_1_Windows__CGraphics__CRectInt32 ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CRectInt32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CRectInt32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CRectInt32_USE
#define DEF___FIIterable_1_Windows__CGraphics__CRectInt32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d6222360-b82e-5eed-9eab-2e275b36e47e"))
IIterable<struct ABI::Windows::Graphics::RectInt32> : IIterable_impl<struct ABI::Windows::Graphics::RectInt32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.RectInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Graphics::RectInt32> __FIIterable_1_Windows__CGraphics__CRectInt32_t;
#define __FIIterable_1_Windows__CGraphics__CRectInt32 ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CRectInt32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CRectInt32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CRectInt32_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CRectInt32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4f5e243f-3812-5200-b70c-30dcfc61717b"))
IVectorView<struct ABI::Windows::Graphics::RectInt32> : IVectorView_impl<struct ABI::Windows::Graphics::RectInt32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.RectInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Graphics::RectInt32> __FIVectorView_1_Windows__CGraphics__CRectInt32_t;
#define __FIVectorView_1_Windows__CGraphics__CRectInt32 ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CRectInt32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CRectInt32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                class Direct3D11CaptureFramePool;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51a947f7-79cf-5a3e-a3a5-1289cfa6dfe8"))
ITypedEventHandler<ABI::Windows::Graphics::Capture::Direct3D11CaptureFramePool*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Capture::Direct3D11CaptureFramePool*, ABI::Windows::Graphics::Capture::IDirect3D11CaptureFramePool*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Capture.Direct3D11CaptureFramePool, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Capture::Direct3D11CaptureFramePool*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e9c610c0-a68c-5bd9-8021-8589346eeee2"))
ITypedEventHandler<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*, ABI::Windows::Graphics::Capture::IGraphicsCaptureItem*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Capture.GraphicsCaptureItem, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Capture::GraphicsCaptureItem*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DDevice;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

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
        namespace Graphics {
            namespace DirectX {
                typedef enum DirectXPixelFormat : int DirectXPixelFormat;
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct DisplayId DisplayId;
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct SizeInt32 SizeInt32;
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            class DispatcherQueue;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IDispatcherQueue;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIDispatcherQueue ABI::Windows::System::IDispatcherQueue

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                class Visual;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface IVisual;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CIVisual ABI::Windows::UI::Composition::IVisual

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                typedef enum GraphicsCaptureAccessKind : int GraphicsCaptureAccessKind;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                typedef enum GraphicsCaptureDirtyRegionMode : int GraphicsCaptureDirtyRegionMode;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                class Direct3D11CaptureFrame;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                class GraphicsCaptureSession;
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Capture.GraphicsCaptureAccessKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                enum GraphicsCaptureAccessKind : int
                {
                    GraphicsCaptureAccessKind_Borderless = 0,
                    GraphicsCaptureAccessKind_Programmatic = 1,
                };
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Struct Windows.Graphics.Capture.GraphicsCaptureDirtyRegionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                enum GraphicsCaptureDirtyRegionMode : int
                {
                    GraphicsCaptureDirtyRegionMode_ReportOnly = 0,
                    GraphicsCaptureDirtyRegionMode_ReportAndRender = 1,
                };
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFrame[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFrame";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("fa50c623-38da-4b32-acf3-fa9734ad800e")
                IDirect3D11CaptureFrame : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Surface(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemRelativeTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContentSize(
                        ABI::Windows::Graphics::SizeInt32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDirect3D11CaptureFrame = __uuidof(IDirect3D11CaptureFrame);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFrame2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFrame2[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFrame2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("37869cfa-2b48-5ebf-9afb-dffd805defdb")
                IDirect3D11CaptureFrame2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DirtyRegions(
                        __FIVectorView_1_Windows__CGraphics__CRectInt32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DirtyRegionMode(
                        ABI::Windows::Graphics::Capture::GraphicsCaptureDirtyRegionMode* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDirect3D11CaptureFrame2 = __uuidof(IDirect3D11CaptureFrame2);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFramePool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFramePool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFramePool[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFramePool";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("24eb6d22-1975-422e-82e7-780dbd8ddf24")
                IDirect3D11CaptureFramePool : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Recreate(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice* device,
                        ABI::Windows::Graphics::DirectX::DirectXPixelFormat pixelFormat,
                        INT32 numberOfBuffers,
                        ABI::Windows::Graphics::SizeInt32 size
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetNextFrame(
                        ABI::Windows::Graphics::Capture::IDirect3D11CaptureFrame** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_FrameArrived(
                        __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_FrameArrived(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCaptureSession(
                        ABI::Windows::Graphics::Capture::IGraphicsCaptureItem* item,
                        ABI::Windows::Graphics::Capture::IGraphicsCaptureSession** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DispatcherQueue(
                        ABI::Windows::System::IDispatcherQueue** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDirect3D11CaptureFramePool = __uuidof(IDirect3D11CaptureFramePool);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFramePool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFramePoolStatics[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("7784056a-67aa-4d53-ae54-1088d5a8ca21")
                IDirect3D11CaptureFramePoolStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice* device,
                        ABI::Windows::Graphics::DirectX::DirectXPixelFormat pixelFormat,
                        INT32 numberOfBuffers,
                        ABI::Windows::Graphics::SizeInt32 size,
                        ABI::Windows::Graphics::Capture::IDirect3D11CaptureFramePool** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDirect3D11CaptureFramePoolStatics = __uuidof(IDirect3D11CaptureFramePoolStatics);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFramePool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFramePoolStatics2[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("589b103f-6bbc-5df5-a991-02e28b3b66d5")
                IDirect3D11CaptureFramePoolStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFreeThreaded(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice* device,
                        ABI::Windows::Graphics::DirectX::DirectXPixelFormat pixelFormat,
                        INT32 numberOfBuffers,
                        ABI::Windows::Graphics::SizeInt32 size,
                        ABI::Windows::Graphics::Capture::IDirect3D11CaptureFramePool** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDirect3D11CaptureFramePoolStatics2 = __uuidof(IDirect3D11CaptureFramePoolStatics2);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureAccessStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureAccess
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureAccessStatics[] = L"Windows.Graphics.Capture.IGraphicsCaptureAccessStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("743ed370-06ec-5040-a58a-901f0f757095")
                IGraphicsCaptureAccessStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        ABI::Windows::Graphics::Capture::GraphicsCaptureAccessKind request,
                        __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureAccessStatics = __uuidof(IGraphicsCaptureAccessStatics);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureItem[] = L"Windows.Graphics.Capture.IGraphicsCaptureItem";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("79c3f95b-31f7-4ec2-a464-632ef5d30760")
                IGraphicsCaptureItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        ABI::Windows::Graphics::SizeInt32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Closed(
                        __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureItem = __uuidof(IGraphicsCaptureItem);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureItemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureItemStatics[] = L"Windows.Graphics.Capture.IGraphicsCaptureItemStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("a87ebea5-457c-5788-ab47-0cf1d3637e74")
                IGraphicsCaptureItemStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromVisual(
                        ABI::Windows::UI::Composition::IVisual* visual,
                        ABI::Windows::Graphics::Capture::IGraphicsCaptureItem** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureItemStatics = __uuidof(IGraphicsCaptureItemStatics);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureItemStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureItemStatics2[] = L"Windows.Graphics.Capture.IGraphicsCaptureItemStatics2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("3b92acc9-e584-5862-bf5c-9c316c6d2dbb")
                IGraphicsCaptureItemStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryCreateFromWindowId(
                        ABI::Windows::UI::WindowId windowId,
                        ABI::Windows::Graphics::Capture::IGraphicsCaptureItem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCreateFromDisplayId(
                        ABI::Windows::Graphics::DisplayId displayId,
                        ABI::Windows::Graphics::Capture::IGraphicsCaptureItem** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureItemStatics2 = __uuidof(IGraphicsCaptureItemStatics2);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCapturePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCapturePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCapturePicker[] = L"Windows.Graphics.Capture.IGraphicsCapturePicker";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("5a1711b3-ad79-4b4a-9336-1318fdde3539")
                IGraphicsCapturePicker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE PickSingleItemAsync(
                        __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCapturePicker = __uuidof(IGraphicsCapturePicker);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("814e42a9-f70f-4ad7-939b-fddcc6eb880d")
                IGraphicsCaptureSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE StartCapture(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureSession = __uuidof(IGraphicsCaptureSession);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession2[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("2c39ae40-7d2e-5044-804e-8b6799d4cf9e")
                IGraphicsCaptureSession2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsCursorCaptureEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsCursorCaptureEnabled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureSession2 = __uuidof(IGraphicsCaptureSession2);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession3[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("f2cdd966-22ae-5ea1-9596-3a289344c3be")
                IGraphicsCaptureSession3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsBorderRequired(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsBorderRequired(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureSession3 = __uuidof(IGraphicsCaptureSession3);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession4[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession4";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("ae99813c-c257-5759-8ed0-668c9b557ed4")
                IGraphicsCaptureSession4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DirtyRegionMode(
                        ABI::Windows::Graphics::Capture::GraphicsCaptureDirtyRegionMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DirtyRegionMode(
                        ABI::Windows::Graphics::Capture::GraphicsCaptureDirtyRegionMode value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureSession4 = __uuidof(IGraphicsCaptureSession4);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession5[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession5";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("67c0ea62-1f85-5061-925a-239be0ac09cb")
                IGraphicsCaptureSession5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MinUpdateInterval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MinUpdateInterval(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureSession5 = __uuidof(IGraphicsCaptureSession5);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession6[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession6";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("d7419236-be20-5e9f-bcd6-c4e98fd6afdc")
                IGraphicsCaptureSession6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeSecondaryWindows(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IncludeSecondaryWindows(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureSession6 = __uuidof(IGraphicsCaptureSession6);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSessionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSessionStatics[] = L"Windows.Graphics.Capture.IGraphicsCaptureSessionStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Capture {
                MIDL_INTERFACE("2224a540-5974-49aa-b232-0882536f4cb5")
                IGraphicsCaptureSessionStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGraphicsCaptureSessionStatics = __uuidof(IGraphicsCaptureSessionStatics);
            } /* Capture */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.Direct3D11CaptureFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IDirect3D11CaptureFrame ** Default Interface **
 *    Windows.Graphics.Capture.IDirect3D11CaptureFrame2
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_Direct3D11CaptureFrame_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_Direct3D11CaptureFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_Direct3D11CaptureFrame[] = L"Windows.Graphics.Capture.Direct3D11CaptureFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.Direct3D11CaptureFramePool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IDirect3D11CaptureFramePool ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_Direct3D11CaptureFramePool_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_Direct3D11CaptureFramePool_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_Direct3D11CaptureFramePool[] = L"Windows.Graphics.Capture.Direct3D11CaptureFramePool";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.GraphicsCaptureAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Capture.IGraphicsCaptureAccessStatics interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureAccess_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureAccess_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_GraphicsCaptureAccess[] = L"Windows.Graphics.Capture.GraphicsCaptureAccess";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Graphics.Capture.GraphicsCaptureItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Capture.IGraphicsCaptureItemStatics2 interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Capture.IGraphicsCaptureItemStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IGraphicsCaptureItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureItem_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_GraphicsCaptureItem[] = L"Windows.Graphics.Capture.GraphicsCaptureItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.GraphicsCapturePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IGraphicsCapturePicker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCapturePicker_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCapturePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_GraphicsCapturePicker[] = L"Windows.Graphics.Capture.GraphicsCapturePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.GraphicsCaptureSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Capture.IGraphicsCaptureSessionStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IGraphicsCaptureSession ** Default Interface **
 *    Windows.Graphics.Capture.IGraphicsCaptureSession2
 *    Windows.Graphics.Capture.IGraphicsCaptureSession3
 *    Windows.Graphics.Capture.IGraphicsCaptureSession4
 *    Windows.Graphics.Capture.IGraphicsCaptureSession5
 *    Windows.Graphics.Capture.IGraphicsCaptureSession6
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureSession_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_GraphicsCaptureSession[] = L"Windows.Graphics.Capture.GraphicsCaptureSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2 __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2 __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2 __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2 __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3 __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4 __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5 __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6 __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics;

#endif // ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItemVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* This,
        __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItemVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

typedef enum __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        enum __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef struct __x_ABI_CWindows_CGraphics_CRectInt32 __x_ABI_CWindows_CGraphics_CRectInt32;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CRectInt32 __FIIterator_1_Windows__CGraphics__CRectInt32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CRectInt32;

typedef struct __FIIterator_1_Windows__CGraphics__CRectInt32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CRectInt32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CRectInt32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        struct __x_ABI_CWindows_CGraphics_CRectInt32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CRectInt32* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CGraphics_CRectInt32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CRectInt32Vtbl;

interface __FIIterator_1_Windows__CGraphics__CRectInt32
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CRectInt32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CRectInt32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CRectInt32_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CRectInt32 __FIIterable_1_Windows__CGraphics__CRectInt32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CRectInt32;

typedef struct __FIIterable_1_Windows__CGraphics__CRectInt32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CRectInt32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CRectInt32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CRectInt32* This,
        __FIIterator_1_Windows__CGraphics__CRectInt32** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CRectInt32Vtbl;

interface __FIIterable_1_Windows__CGraphics__CRectInt32
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CRectInt32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CRectInt32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CRectInt32_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CRectInt32 __FIVectorView_1_Windows__CGraphics__CRectInt32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CRectInt32;

typedef struct __FIVectorView_1_Windows__CGraphics__CRectInt32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This,
        UINT32 index,
        struct __x_ABI_CWindows_CGraphics_CRectInt32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This,
        struct __x_ABI_CWindows_CGraphics_CRectInt32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CRectInt32* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CGraphics_CRectInt32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CRectInt32Vtbl;

interface __FIVectorView_1_Windows__CGraphics__CRectInt32
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CRectInt32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CRectInt32_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CRectInt32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat;

typedef struct __x_ABI_CWindows_CGraphics_CDisplayId __x_ABI_CWindows_CGraphics_CDisplayId;

typedef struct __x_ABI_CWindows_CGraphics_CSizeInt32 __x_ABI_CWindows_CGraphics_CSizeInt32;

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIDispatcherQueue __x_ABI_CWindows_CSystem_CIDispatcherQueue;

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CIVisual __x_ABI_CWindows_CUI_CComposition_CIVisual;

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

typedef enum __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureAccessKind __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureAccessKind;

typedef enum __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureDirtyRegionMode __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureDirtyRegionMode;

/*
 *
 * Struct Windows.Graphics.Capture.GraphicsCaptureAccessKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
enum __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureAccessKind
{
    GraphicsCaptureAccessKind_Borderless = 0,
    GraphicsCaptureAccessKind_Programmatic = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Struct Windows.Graphics.Capture.GraphicsCaptureDirtyRegionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureDirtyRegionMode
{
    GraphicsCaptureDirtyRegionMode_ReportOnly = 0,
    GraphicsCaptureDirtyRegionMode_ReportAndRender = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFrame[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFrame";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Surface)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativeTime)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentSize)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrameVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_get_Surface(This, value) \
    ((This)->lpVtbl->get_Surface(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_get_SystemRelativeTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativeTime(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_get_ContentSize(This, value) \
    ((This)->lpVtbl->get_ContentSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFrame2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFrame2[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFrame2";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DirtyRegions)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2* This,
        __FIVectorView_1_Windows__CGraphics__CRectInt32** value);
    HRESULT (STDMETHODCALLTYPE* get_DirtyRegionMode)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2* This,
        enum __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureDirtyRegionMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2Vtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_get_DirtyRegions(This, value) \
    ((This)->lpVtbl->get_DirtyRegions(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_get_DirtyRegionMode(This, value) \
    ((This)->lpVtbl->get_DirtyRegionMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFramePool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFramePool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFramePool[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFramePool";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Recreate)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* device,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat pixelFormat,
        INT32 numberOfBuffers,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32 size);
    HRESULT (STDMETHODCALLTYPE* TryGetNextFrame)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFrame** result);
    HRESULT (STDMETHODCALLTYPE* add_FrameArrived)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        __FITypedEventHandler_2_Windows__CGraphics__CCapture__CDirect3D11CaptureFramePool_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FrameArrived)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CreateCaptureSession)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* item,
        __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession** result);
    HRESULT (STDMETHODCALLTYPE* get_DispatcherQueue)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool* This,
        __x_ABI_CWindows_CSystem_CIDispatcherQueue** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_Recreate(This, device, pixelFormat, numberOfBuffers, size) \
    ((This)->lpVtbl->Recreate(This, device, pixelFormat, numberOfBuffers, size))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_TryGetNextFrame(This, result) \
    ((This)->lpVtbl->TryGetNextFrame(This, result))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_add_FrameArrived(This, handler, token) \
    ((This)->lpVtbl->add_FrameArrived(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_remove_FrameArrived(This, token) \
    ((This)->lpVtbl->remove_FrameArrived(This, token))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_CreateCaptureSession(This, item, result) \
    ((This)->lpVtbl->CreateCaptureSession(This, item, result))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_get_DispatcherQueue(This, value) \
    ((This)->lpVtbl->get_DispatcherQueue(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFramePool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFramePoolStatics[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* device,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat pixelFormat,
        INT32 numberOfBuffers,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32 size,
        __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_Create(This, device, pixelFormat, numberOfBuffers, size, result) \
    ((This)->lpVtbl->Create(This, device, pixelFormat, numberOfBuffers, size, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.Direct3D11CaptureFramePool
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IDirect3D11CaptureFramePoolStatics2[] = L"Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFreeThreaded)(__x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* device,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat pixelFormat,
        INT32 numberOfBuffers,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32 size,
        __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePool** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2Vtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_CreateFreeThreaded(This, device, pixelFormat, numberOfBuffers, size, result) \
    ((This)->lpVtbl->CreateFreeThreaded(This, device, pixelFormat, numberOfBuffers, size, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIDirect3D11CaptureFramePoolStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureAccessStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureAccess
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureAccessStatics[] = L"Windows.Graphics.Capture.IGraphicsCaptureAccessStatics";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics* This,
        enum __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureAccessKind request,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_RequestAccessAsync(This, request, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, request, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureAccessStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureItem[] = L"Windows.Graphics.Capture.IGraphicsCaptureItem";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This,
        __FITypedEventHandler_2_Windows__CGraphics__CCapture__CGraphicsCaptureItem_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_add_Closed(This, handler, token) \
    ((This)->lpVtbl->add_Closed(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_remove_Closed(This, token) \
    ((This)->lpVtbl->remove_Closed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureItemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureItemStatics[] = L"Windows.Graphics.Capture.IGraphicsCaptureItemStatics";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromVisual)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisual* visual,
        __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_CreateFromVisual(This, visual, result) \
    ((This)->lpVtbl->CreateFromVisual(This, visual, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureItemStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureItemStatics2[] = L"Windows.Graphics.Capture.IGraphicsCaptureItemStatics2";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCreateFromWindowId)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2* This,
        struct __x_ABI_CWindows_CUI_CWindowId windowId,
        __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem** result);
    HRESULT (STDMETHODCALLTYPE* TryCreateFromDisplayId)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2* This,
        struct __x_ABI_CWindows_CGraphics_CDisplayId displayId,
        __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItem** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2Vtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_TryCreateFromWindowId(This, windowId, result) \
    ((This)->lpVtbl->TryCreateFromWindowId(This, windowId, result))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_TryCreateFromDisplayId(This, displayId, result) \
    ((This)->lpVtbl->TryCreateFromDisplayId(This, displayId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureItemStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCapturePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCapturePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCapturePicker[] = L"Windows.Graphics.Capture.IGraphicsCapturePicker";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePickerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* PickSingleItemAsync)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker* This,
        __FIAsyncOperation_1_Windows__CGraphics__CCapture__CGraphicsCaptureItem** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePickerVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePickerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_PickSingleItemAsync(This, operation) \
    ((This)->lpVtbl->PickSingleItemAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCapturePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartCapture)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession* This);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_StartCapture(This) \
    ((This)->lpVtbl->StartCapture(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession2[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession2";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsCursorCaptureEnabled)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCursorCaptureEnabled)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2Vtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_get_IsCursorCaptureEnabled(This, value) \
    ((This)->lpVtbl->get_IsCursorCaptureEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_put_IsCursorCaptureEnabled(This, value) \
    ((This)->lpVtbl->put_IsCursorCaptureEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession3[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession3";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsBorderRequired)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsBorderRequired)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3Vtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_get_IsBorderRequired(This, value) \
    ((This)->lpVtbl->get_IsBorderRequired(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_put_IsBorderRequired(This, value) \
    ((This)->lpVtbl->put_IsBorderRequired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession4[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession4";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DirtyRegionMode)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4* This,
        enum __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureDirtyRegionMode* value);
    HRESULT (STDMETHODCALLTYPE* put_DirtyRegionMode)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4* This,
        enum __x_ABI_CWindows_CGraphics_CCapture_CGraphicsCaptureDirtyRegionMode value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4Vtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_get_DirtyRegionMode(This, value) \
    ((This)->lpVtbl->get_DirtyRegionMode(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_put_DirtyRegionMode(This, value) \
    ((This)->lpVtbl->put_DirtyRegionMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession5[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession5";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MinUpdateInterval)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_MinUpdateInterval)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5Vtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_get_MinUpdateInterval(This, value) \
    ((This)->lpVtbl->get_MinUpdateInterval(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_put_MinUpdateInterval(This, value) \
    ((This)->lpVtbl->put_MinUpdateInterval(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSession6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSession6[] = L"Windows.Graphics.Capture.IGraphicsCaptureSession6";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IncludeSecondaryWindows)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeSecondaryWindows)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6Vtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_get_IncludeSecondaryWindows(This, value) \
    ((This)->lpVtbl->get_IncludeSecondaryWindows(This, value))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_put_IncludeSecondaryWindows(This, value) \
    ((This)->lpVtbl->put_IncludeSecondaryWindows(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSession6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Capture.IGraphicsCaptureSessionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Capture.GraphicsCaptureSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Capture_IGraphicsCaptureSessionStatics[] = L"Windows.Graphics.Capture.IGraphicsCaptureSessionStatics";
typedef struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CCapture_CIGraphicsCaptureSessionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.Direct3D11CaptureFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IDirect3D11CaptureFrame ** Default Interface **
 *    Windows.Graphics.Capture.IDirect3D11CaptureFrame2
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_Direct3D11CaptureFrame_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_Direct3D11CaptureFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_Direct3D11CaptureFrame[] = L"Windows.Graphics.Capture.Direct3D11CaptureFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.Direct3D11CaptureFramePool
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IDirect3D11CaptureFramePool ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_Direct3D11CaptureFramePool_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_Direct3D11CaptureFramePool_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_Direct3D11CaptureFramePool[] = L"Windows.Graphics.Capture.Direct3D11CaptureFramePool";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.GraphicsCaptureAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Capture.IGraphicsCaptureAccessStatics interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureAccess_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureAccess_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_GraphicsCaptureAccess[] = L"Windows.Graphics.Capture.GraphicsCaptureAccess";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Graphics.Capture.GraphicsCaptureItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Capture.IGraphicsCaptureItemStatics2 interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Capture.IGraphicsCaptureItemStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IGraphicsCaptureItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureItem_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_GraphicsCaptureItem[] = L"Windows.Graphics.Capture.GraphicsCaptureItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.GraphicsCapturePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IGraphicsCapturePicker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCapturePicker_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCapturePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_GraphicsCapturePicker[] = L"Windows.Graphics.Capture.GraphicsCapturePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Capture.GraphicsCaptureSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Capture.IGraphicsCaptureSessionStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Capture.IGraphicsCaptureSession ** Default Interface **
 *    Windows.Graphics.Capture.IGraphicsCaptureSession2
 *    Windows.Graphics.Capture.IGraphicsCaptureSession3
 *    Windows.Graphics.Capture.IGraphicsCaptureSession4
 *    Windows.Graphics.Capture.IGraphicsCaptureSession5
 *    Windows.Graphics.Capture.IGraphicsCaptureSession6
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureSession_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Capture_GraphicsCaptureSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Capture_GraphicsCaptureSession[] = L"Windows.Graphics.Capture.GraphicsCaptureSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egraphics2Ecapture_p_h__

#endif // __windows2Egraphics2Ecapture_h__
