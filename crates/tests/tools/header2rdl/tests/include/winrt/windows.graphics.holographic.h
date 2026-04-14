
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
#ifndef __windows2Egraphics2Eholographic_h__
#define __windows2Egraphics2Eholographic_h__
#ifndef __windows2Egraphics2Eholographic_p_h__
#define __windows2Egraphics2Eholographic_p_h__


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

#if !defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.Graphics.DirectX.h"
#include "Windows.Graphics.DirectX.Direct3D11.h"
#include "Windows.Perception.h"
#include "Windows.Perception.Spatial.h"
#include "Windows.UI.Core.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCamera;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera ABI::Windows::Graphics::Holographic::IHolographicCamera

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCamera2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2 ABI::Windows::Graphics::Holographic::IHolographicCamera2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCamera3;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3 ABI::Windows::Graphics::Holographic::IHolographicCamera3

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCamera4;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4 ABI::Windows::Graphics::Holographic::IHolographicCamera4

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCamera5;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5 ABI::Windows::Graphics::Holographic::IHolographicCamera5

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCamera6;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6 ABI::Windows::Graphics::Holographic::IHolographicCamera6

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCameraPose;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose ABI::Windows::Graphics::Holographic::IHolographicCameraPose

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCameraPose2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2 ABI::Windows::Graphics::Holographic::IHolographicCameraPose2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCameraRenderingParameters;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters ABI::Windows::Graphics::Holographic::IHolographicCameraRenderingParameters

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCameraRenderingParameters2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2 ABI::Windows::Graphics::Holographic::IHolographicCameraRenderingParameters2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCameraRenderingParameters3;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3 ABI::Windows::Graphics::Holographic::IHolographicCameraRenderingParameters3

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCameraRenderingParameters4;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4 ABI::Windows::Graphics::Holographic::IHolographicCameraRenderingParameters4

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicCameraViewportParameters;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters ABI::Windows::Graphics::Holographic::IHolographicCameraViewportParameters

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicDisplay;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay ABI::Windows::Graphics::Holographic::IHolographicDisplay

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicDisplay2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2 ABI::Windows::Graphics::Holographic::IHolographicDisplay2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicDisplay3;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3 ABI::Windows::Graphics::Holographic::IHolographicDisplay3

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicDisplayStatics;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics ABI::Windows::Graphics::Holographic::IHolographicDisplayStatics

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFrame;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame ABI::Windows::Graphics::Holographic::IHolographicFrame

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFrame2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2 ABI::Windows::Graphics::Holographic::IHolographicFrame2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFrame3;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3 ABI::Windows::Graphics::Holographic::IHolographicFrame3

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFramePrediction;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction ABI::Windows::Graphics::Holographic::IHolographicFramePrediction

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFramePresentationMonitor;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor ABI::Windows::Graphics::Holographic::IHolographicFramePresentationMonitor

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFramePresentationReport;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport ABI::Windows::Graphics::Holographic::IHolographicFramePresentationReport

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFrameRenderingReport;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport ABI::Windows::Graphics::Holographic::IHolographicFrameRenderingReport

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFrameScanoutMonitor;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor ABI::Windows::Graphics::Holographic::IHolographicFrameScanoutMonitor

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicFrameScanoutReport;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport ABI::Windows::Graphics::Holographic::IHolographicFrameScanoutReport

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicQuadLayer;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer ABI::Windows::Graphics::Holographic::IHolographicQuadLayer

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicQuadLayerFactory;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory ABI::Windows::Graphics::Holographic::IHolographicQuadLayerFactory

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicQuadLayerUpdateParameters;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters ABI::Windows::Graphics::Holographic::IHolographicQuadLayerUpdateParameters

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicQuadLayerUpdateParameters2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2 ABI::Windows::Graphics::Holographic::IHolographicQuadLayerUpdateParameters2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicSpace;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace ABI::Windows::Graphics::Holographic::IHolographicSpace

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicSpace2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2 ABI::Windows::Graphics::Holographic::IHolographicSpace2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicSpace3;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3 ABI::Windows::Graphics::Holographic::IHolographicSpace3

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicSpaceCameraAddedEventArgs;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs ABI::Windows::Graphics::Holographic::IHolographicSpaceCameraAddedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicSpaceCameraRemovedEventArgs;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs ABI::Windows::Graphics::Holographic::IHolographicSpaceCameraRemovedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicSpaceStatics;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics ABI::Windows::Graphics::Holographic::IHolographicSpaceStatics

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicSpaceStatics2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2 ABI::Windows::Graphics::Holographic::IHolographicSpaceStatics2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicSpaceStatics3;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3 ABI::Windows::Graphics::Holographic::IHolographicSpaceStatics3

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicViewConfiguration;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration ABI::Windows::Graphics::Holographic::IHolographicViewConfiguration

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                interface IHolographicViewConfiguration2;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2 ABI::Windows::Graphics::Holographic::IHolographicViewConfiguration2

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                typedef enum DirectXPixelFormat : int DirectXPixelFormat;
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#define DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ea016190-ac80-5840-8f58-ff434c7b2907"))
IIterator<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> : IIterator_impl<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.DirectX.DirectXPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t;
#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#define DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3908f2c6-1aee-5129-b9a6-2a6e01d9507e"))
IIterable<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> : IIterable_impl<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.DirectX.DirectXPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t;
#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicCamera;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE
#define DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6acc8576-2fea-561d-84dd-4a1ab05fc7ed"))
IIterator<ABI::Windows::Graphics::Holographic::HolographicCamera*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicCamera*, ABI::Windows::Graphics::Holographic::IHolographicCamera*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Holographic.HolographicCamera>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Holographic::HolographicCamera*> __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_t;
#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE
#define DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b2afd154-8db0-5bb2-ad7a-684afd479264"))
IIterable<ABI::Windows::Graphics::Holographic::HolographicCamera*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicCamera*, ABI::Windows::Graphics::Holographic::IHolographicCamera*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Holographic.HolographicCamera>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Holographic::HolographicCamera*> __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_t;
#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicCameraPose;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE
#define DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("93e27fb4-332b-591e-ae6b-6192fa0a1009"))
IIterator<ABI::Windows::Graphics::Holographic::HolographicCameraPose*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicCameraPose*, ABI::Windows::Graphics::Holographic::IHolographicCameraPose*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Holographic.HolographicCameraPose>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Holographic::HolographicCameraPose*> __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_t;
#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE
#define DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("92111aff-8dcc-538e-ae3d-31fd252a0ad5"))
IIterable<ABI::Windows::Graphics::Holographic::HolographicCameraPose*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicCameraPose*, ABI::Windows::Graphics::Holographic::IHolographicCameraPose*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Holographic.HolographicCameraPose>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Holographic::HolographicCameraPose*> __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_t;
#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef enum HolographicDepthReprojectionMethod : int HolographicDepthReprojectionMethod;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE
#define DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2486838-8e17-52a2-b3b2-8db3de6412b7"))
IIterator<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod> : IIterator_impl<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Holographic.HolographicDepthReprojectionMethod>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod> __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_t;
#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE
#define DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2803c19f-8082-52c0-aad1-68ab51ed287f"))
IIterable<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod> : IIterable_impl<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Holographic.HolographicDepthReprojectionMethod>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod> __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_t;
#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicFramePresentationReport;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE
#define DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f81cb835-d690-55d7-adfb-7b92b96e29a0"))
IIterator<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*, ABI::Windows::Graphics::Holographic::IHolographicFramePresentationReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Holographic.HolographicFramePresentationReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*> __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_t;
#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE
#define DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d0fe1cdf-33ad-5051-8c5b-ab9a2b2c24bf"))
IIterable<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*, ABI::Windows::Graphics::Holographic::IHolographicFramePresentationReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Holographic.HolographicFramePresentationReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*> __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_t;
#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicFrameScanoutReport;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE
#define DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6a7de579-05db-58f6-a371-97fd19922130"))
IIterator<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*, ABI::Windows::Graphics::Holographic::IHolographicFrameScanoutReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Holographic.HolographicFrameScanoutReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*> __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_t;
#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE
#define DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cfa9ea14-4803-5001-9eda-cb4ffcd270e9"))
IIterable<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*, ABI::Windows::Graphics::Holographic::IHolographicFrameScanoutReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Holographic.HolographicFrameScanoutReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*> __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_t;
#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicQuadLayer;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE
#define DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("85765170-495b-541c-aef0-7492856de3df"))
IIterator<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*, ABI::Windows::Graphics::Holographic::IHolographicQuadLayer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Holographic.HolographicQuadLayer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*> __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_t;
#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE
#define DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("84744661-94de-5866-a15d-9efb19a99a54"))
IIterable<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*, ABI::Windows::Graphics::Holographic::IHolographicQuadLayer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Holographic.HolographicQuadLayer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*> __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_t;
#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1edda1c2-0f6e-516c-80b8-7687dcd1280e"))
IVectorView<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> : IVectorView_impl<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.DirectX.DirectXPixelFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Graphics::DirectX::DirectXPixelFormat> __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t;
#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("01d6c0ae-ada5-50b0-8562-41fb1205bb4a"))
IVectorView<ABI::Windows::Graphics::Holographic::HolographicCamera*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicCamera*, ABI::Windows::Graphics::Holographic::IHolographicCamera*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Holographic.HolographicCamera>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Holographic::HolographicCamera*> __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_t;
#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("17c5dfb1-6e87-5a17-a791-ac07f8ee9292"))
IVectorView<ABI::Windows::Graphics::Holographic::HolographicCameraPose*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicCameraPose*, ABI::Windows::Graphics::Holographic::IHolographicCameraPose*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Holographic.HolographicCameraPose>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Holographic::HolographicCameraPose*> __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_t;
#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7ac6dc9e-ea0b-594a-b0ed-2d1764ec58e9"))
IVectorView<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod> : IVectorView_impl<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Holographic.HolographicDepthReprojectionMethod>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod> __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_t;
#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("014f37ba-abc1-5d85-855e-ec053183a635"))
IVectorView<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*, ABI::Windows::Graphics::Holographic::IHolographicFramePresentationReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Holographic.HolographicFramePresentationReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Holographic::HolographicFramePresentationReport*> __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_t;
#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a127f0f1-fb53-50e2-8a98-0ecaea62e32b"))
IVectorView<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*, ABI::Windows::Graphics::Holographic::IHolographicFrameScanoutReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Holographic.HolographicFrameScanoutReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*> __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_t;
#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1f51ecdf-cf2d-5b7e-aae9-d6628a518dbe"))
IVectorView<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*, ABI::Windows::Graphics::Holographic::IHolographicQuadLayer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Holographic.HolographicQuadLayer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*> __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_t;
#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE
#define DEF___FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a8081e2b-440d-53fc-a310-cf8a07ea0935"))
IVector<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*, ABI::Windows::Graphics::Holographic::IHolographicFrameScanoutReport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Holographic.HolographicFrameScanoutReport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Holographic::HolographicFrameScanoutReport*> __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_t;
#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE
#define DEF___FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("da24dfcc-4c54-5193-921d-c685b57de559"))
IVector<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*, ABI::Windows::Graphics::Holographic::IHolographicQuadLayer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Holographic.HolographicQuadLayer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Holographic::HolographicQuadLayer*> __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_t;
#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000


#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef struct HolographicStereoTransform HolographicStereoTransform;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_USE
#define DEF___FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6e67ce78-cc67-52c0-b635-991db0bff5ca"))
IReference<struct ABI::Windows::Graphics::Holographic::HolographicStereoTransform> : IReference_impl<struct ABI::Windows::Graphics::Holographic::HolographicStereoTransform>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Graphics.Holographic.HolographicStereoTransform>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Graphics::Holographic::HolographicStereoTransform> __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_t;
#define __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform ABI::Windows::Foundation::__FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef struct SpatialBoundingFrustum SpatialBoundingFrustum;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_USE
#define DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f434face-0c36-5749-a8a0-0bb6ce78a614"))
IReference<struct ABI::Windows::Perception::Spatial::SpatialBoundingFrustum> : IReference_impl<struct ABI::Windows::Perception::Spatial::SpatialBoundingFrustum>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Perception.Spatial.SpatialBoundingFrustum>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Perception::Spatial::SpatialBoundingFrustum> __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_t;
#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum ABI::Windows::Foundation::__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicSpace;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("67aae2f2-42d8-5503-9131-deeb45a6ca03"))
ITypedEventHandler<ABI::Windows::Graphics::Holographic::HolographicSpace*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicSpace*, ABI::Windows::Graphics::Holographic::IHolographicSpace*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Holographic.HolographicSpace, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Holographic::HolographicSpace*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicSpaceCameraAddedEventArgs;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("71d80b7c-1d27-5102-83d1-4f0efc7c9d6f"))
ITypedEventHandler<ABI::Windows::Graphics::Holographic::HolographicSpace*, ABI::Windows::Graphics::Holographic::HolographicSpaceCameraAddedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicSpace*, ABI::Windows::Graphics::Holographic::IHolographicSpace*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicSpaceCameraAddedEventArgs*, ABI::Windows::Graphics::Holographic::IHolographicSpaceCameraAddedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Holographic.HolographicSpace, Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Holographic::HolographicSpace*, ABI::Windows::Graphics::Holographic::HolographicSpaceCameraAddedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicSpaceCameraRemovedEventArgs;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("db68cfc3-0874-502a-a3b9-2b1fe86c67be"))
ITypedEventHandler<ABI::Windows::Graphics::Holographic::HolographicSpace*, ABI::Windows::Graphics::Holographic::HolographicSpaceCameraRemovedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicSpace*, ABI::Windows::Graphics::Holographic::IHolographicSpace*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Holographic::HolographicSpaceCameraRemovedEventArgs*, ABI::Windows::Graphics::Holographic::IHolographicSpaceCameraRemovedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Holographic.HolographicSpace, Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Holographic::HolographicSpace*, ABI::Windows::Graphics::Holographic::HolographicSpaceCameraRemovedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

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
            namespace Numerics {
                typedef struct Matrix4x4 Matrix4x4;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Quaternion Quaternion;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Vector2 Vector2;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Vector3 Vector3;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Size Size;
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
        namespace Perception {
            class PerceptionTimestamp;
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            interface IPerceptionTimestamp;
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CIPerceptionTimestamp ABI::Windows::Perception::IPerceptionTimestamp

#endif // ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialLocator;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialLocator;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator ABI::Windows::Perception::Spatial::ISpatialLocator

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreWindow;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindow;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindow ABI::Windows::UI::Core::ICoreWindow

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef enum HolographicFramePresentResult : int HolographicFramePresentResult;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef enum HolographicFramePresentWaitBehavior : int HolographicFramePresentWaitBehavior;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef enum HolographicReprojectionMode : int HolographicReprojectionMode;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef enum HolographicSpaceUserPresence : int HolographicSpaceUserPresence;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef enum HolographicViewConfigurationKind : int HolographicViewConfigurationKind;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef struct HolographicAdapterId HolographicAdapterId;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                typedef struct HolographicFrameId HolographicFrameId;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicCameraRenderingParameters;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicCameraViewportParameters;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicDisplay;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicFrame;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicFramePrediction;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicFramePresentationMonitor;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicFrameRenderingReport;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicFrameScanoutMonitor;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicQuadLayerUpdateParameters;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                class HolographicViewConfiguration;
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicDepthReprojectionMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                enum HolographicDepthReprojectionMethod : int
                {
                    HolographicDepthReprojectionMethod_DepthReprojection = 0,
                    HolographicDepthReprojectionMethod_AutoPlanar = 1,
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicFramePresentResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                enum HolographicFramePresentResult : int
                {
                    HolographicFramePresentResult_Success = 0,
                    HolographicFramePresentResult_DeviceRemoved = 1,
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicFramePresentWaitBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                enum HolographicFramePresentWaitBehavior : int
                {
                    HolographicFramePresentWaitBehavior_WaitForFrameToFinish = 0,
                    HolographicFramePresentWaitBehavior_DoNotWaitForFrameToFinish = 1,
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicReprojectionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                enum HolographicReprojectionMode : int
                {
                    HolographicReprojectionMode_PositionAndOrientation = 0,
                    HolographicReprojectionMode_OrientationOnly = 1,
                    HolographicReprojectionMode_Disabled = 2,
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicSpaceUserPresence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                enum HolographicSpaceUserPresence : int
                {
                    HolographicSpaceUserPresence_Absent = 0,
                    HolographicSpaceUserPresence_PresentPassive = 1,
                    HolographicSpaceUserPresence_PresentActive = 2,
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicViewConfigurationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                enum HolographicViewConfigurationKind : int
                {
                    HolographicViewConfigurationKind_Display = 0,
                    HolographicViewConfigurationKind_PhotoVideoCamera = 1,
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicAdapterId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                struct HolographicAdapterId
                {
                    UINT32 LowPart;
                    INT32 HighPart;
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicFrameId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                struct HolographicFrameId
                {
                    UINT64 Value;
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicStereoTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                struct HolographicStereoTransform
                {
                    ABI::Windows::Foundation::Numerics::Matrix4x4 Left;
                    ABI::Windows::Foundation::Numerics::Matrix4x4 Right;
                };
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera[] = L"Windows.Graphics.Holographic.IHolographicCamera";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("e4e98445-9bed-4980-9ba0-e87680d1cb74")
                IHolographicCamera : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RenderTargetSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ViewportScaleFactor(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ViewportScaleFactor(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsStereo(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetNearPlaneDistance(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetFarPlaneDistance(
                        DOUBLE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCamera = __uuidof(IHolographicCamera);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera2[] = L"Windows.Graphics.Holographic.IHolographicCamera2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("b55b9f1a-ba8c-4f84-ad79-2e7e1e2450f3")
                IHolographicCamera2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LeftViewportParameters(
                        ABI::Windows::Graphics::Holographic::IHolographicCameraViewportParameters** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RightViewportParameters(
                        ABI::Windows::Graphics::Holographic::IHolographicCameraViewportParameters** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Display(
                        ABI::Windows::Graphics::Holographic::IHolographicDisplay** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCamera2 = __uuidof(IHolographicCamera2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicCamera2
 *     Windows.Graphics.Holographic.IHolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera3[] = L"Windows.Graphics.Holographic.IHolographicCamera3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("45aa4fb3-7b59-524e-4a3f-4a6ad6650477")
                IHolographicCamera3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsPrimaryLayerEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsPrimaryLayerEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxQuadLayerCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_QuadLayers(
                        __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCamera3 = __uuidof(IHolographicCamera3);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera4[] = L"Windows.Graphics.Holographic.IHolographicCamera4";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("9a2531d6-4723-4f39-a9a5-9d05181d9b44")
                IHolographicCamera4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanOverrideViewport(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCamera4 = __uuidof(IHolographicCamera4);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera5[] = L"Windows.Graphics.Holographic.IHolographicCamera5";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("229706f2-628d-4ef5-9c08-a63fdd7787c6")
                IHolographicCamera5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsHardwareContentProtectionSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsHardwareContentProtectionEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsHardwareContentProtectionEnabled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCamera5 = __uuidof(IHolographicCamera5);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera6[] = L"Windows.Graphics.Holographic.IHolographicCamera6";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("0209194f-632d-5154-ab52-0b5d15b12505")
                IHolographicCamera6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ViewConfiguration(
                        ABI::Windows::Graphics::Holographic::IHolographicViewConfiguration** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCamera6 = __uuidof(IHolographicCamera6);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraPose[] = L"Windows.Graphics.Holographic.IHolographicCameraPose";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("0d7d7e30-12de-45bd-912b-c7f6561599d1")
                IHolographicCameraPose : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HolographicCamera(
                        ABI::Windows::Graphics::Holographic::IHolographicCamera** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Viewport(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetViewTransform(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProjectionTransform(
                        ABI::Windows::Graphics::Holographic::HolographicStereoTransform* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetCullingFrustum(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetVisibleFrustum(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NearPlaneDistance(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FarPlaneDistance(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCameraPose = __uuidof(IHolographicCameraPose);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraPose2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraPose2[] = L"Windows.Graphics.Holographic.IHolographicCameraPose2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("232be073-5d2d-4560-814e-2697c4fce16b")
                IHolographicCameraPose2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE OverrideViewTransform(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Graphics::Holographic::HolographicStereoTransform coordinateSystemToViewTransform
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OverrideProjectionTransform(
                        ABI::Windows::Graphics::Holographic::HolographicStereoTransform projectionTransform
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OverrideViewport(
                        ABI::Windows::Foundation::Rect leftViewport,
                        ABI::Windows::Foundation::Rect rightViewport
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCameraPose2 = __uuidof(IHolographicCameraPose2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraRenderingParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraRenderingParameters[] = L"Windows.Graphics.Holographic.IHolographicCameraRenderingParameters";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("8eac2ed1-5bf4-4e16-8236-ae0800c11d0d")
                IHolographicCameraRenderingParameters : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetFocusPoint(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Foundation::Numerics::Vector3 position
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetFocusPointWithNormal(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Foundation::Numerics::Vector3 position,
                        ABI::Windows::Foundation::Numerics::Vector3 normal
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetFocusPointWithNormalLinearVelocity(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Foundation::Numerics::Vector3 position,
                        ABI::Windows::Foundation::Numerics::Vector3 normal,
                        ABI::Windows::Foundation::Numerics::Vector3 linearVelocity
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Direct3D11Device(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Direct3D11BackBuffer(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCameraRenderingParameters = __uuidof(IHolographicCameraRenderingParameters);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicCameraRenderingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraRenderingParameters2[] = L"Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("261270e3-b696-4634-94d6-be0681643599")
                IHolographicCameraRenderingParameters2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ReprojectionMode(
                        ABI::Windows::Graphics::Holographic::HolographicReprojectionMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReprojectionMode(
                        ABI::Windows::Graphics::Holographic::HolographicReprojectionMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CommitDirect3D11DepthBuffer(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCameraRenderingParameters2 = __uuidof(IHolographicCameraRenderingParameters2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraRenderingParameters3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2
 *     Windows.Graphics.Holographic.IHolographicCameraRenderingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraRenderingParameters3[] = L"Windows.Graphics.Holographic.IHolographicCameraRenderingParameters3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("b1aa513f-136d-4b06-b9d4-e4b914cd0683")
                IHolographicCameraRenderingParameters3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsContentProtectionEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsContentProtectionEnabled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCameraRenderingParameters3 = __uuidof(IHolographicCameraRenderingParameters3);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraRenderingParameters4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraRenderingParameters4[] = L"Windows.Graphics.Holographic.IHolographicCameraRenderingParameters4";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("0878fa4c-e163-57dc-82b7-c406ab3e0537")
                IHolographicCameraRenderingParameters4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DepthReprojectionMethod(
                        ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DepthReprojectionMethod(
                        ABI::Windows::Graphics::Holographic::HolographicDepthReprojectionMethod value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCameraRenderingParameters4 = __uuidof(IHolographicCameraRenderingParameters4);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraViewportParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraViewportParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraViewportParameters[] = L"Windows.Graphics.Holographic.IHolographicCameraViewportParameters";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("80cdf3f7-842a-41e1-93ed-5692ab1fbb10")
                IHolographicCameraViewportParameters : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HiddenAreaMesh(
                        UINT32* valueLength,
                        ABI::Windows::Foundation::Numerics::Vector2** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VisibleAreaMesh(
                        UINT32* valueLength,
                        ABI::Windows::Foundation::Numerics::Vector2** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicCameraViewportParameters = __uuidof(IHolographicCameraViewportParameters);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicDisplay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicDisplay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicDisplay[] = L"Windows.Graphics.Holographic.IHolographicDisplay";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("9acea414-1d9f-4090-a388-90c06f6eae9c")
                IHolographicDisplay : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxViewportSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsStereo(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOpaque(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AdapterId(
                        ABI::Windows::Graphics::Holographic::HolographicAdapterId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SpatialLocator(
                        ABI::Windows::Perception::Spatial::ISpatialLocator** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicDisplay = __uuidof(IHolographicDisplay);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicDisplay2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicDisplay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicDisplay2[] = L"Windows.Graphics.Holographic.IHolographicDisplay2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("75ac3f82-e755-436c-8d96-4d32d131473e")
                IHolographicDisplay2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RefreshRate(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicDisplay2 = __uuidof(IHolographicDisplay2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicDisplay3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicDisplay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicDisplay3[] = L"Windows.Graphics.Holographic.IHolographicDisplay3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("fc4c6ac6-6480-5008-b29e-157d77c843f7")
                IHolographicDisplay3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryGetViewConfiguration(
                        ABI::Windows::Graphics::Holographic::HolographicViewConfigurationKind kind,
                        ABI::Windows::Graphics::Holographic::IHolographicViewConfiguration** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicDisplay3 = __uuidof(IHolographicDisplay3);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicDisplayStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicDisplay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicDisplayStatics[] = L"Windows.Graphics.Holographic.IHolographicDisplayStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("cb374983-e7b0-4841-8355-3ae5b536e9a4")
                IHolographicDisplayStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Graphics::Holographic::IHolographicDisplay** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicDisplayStatics = __uuidof(IHolographicDisplayStatics);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrame[] = L"Windows.Graphics.Holographic.IHolographicFrame";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("c6988eb6-a8b9-3054-a6eb-d624b6536375")
                IHolographicFrame : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AddedCameras(
                        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemovedCameras(
                        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRenderingParameters(
                        ABI::Windows::Graphics::Holographic::IHolographicCameraPose* cameraPose,
                        ABI::Windows::Graphics::Holographic::IHolographicCameraRenderingParameters** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentPrediction(
                        ABI::Windows::Graphics::Holographic::IHolographicFramePrediction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateCurrentPrediction(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PresentUsingCurrentPrediction(
                        ABI::Windows::Graphics::Holographic::HolographicFramePresentResult* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PresentUsingCurrentPredictionWithBehavior(
                        ABI::Windows::Graphics::Holographic::HolographicFramePresentWaitBehavior waitBehavior,
                        ABI::Windows::Graphics::Holographic::HolographicFramePresentResult* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WaitForFrameToFinish(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFrame = __uuidof(IHolographicFrame);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrame2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrame
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrame2[] = L"Windows.Graphics.Holographic.IHolographicFrame2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("283f37bf-3bf2-5e91-6633-870574e6f217")
                IHolographicFrame2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetQuadLayerUpdateParameters(
                        ABI::Windows::Graphics::Holographic::IHolographicQuadLayer* layer,
                        ABI::Windows::Graphics::Holographic::IHolographicQuadLayerUpdateParameters** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFrame2 = __uuidof(IHolographicFrame2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrame3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrame3[] = L"Windows.Graphics.Holographic.IHolographicFrame3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("e5e964c9-8a27-55d3-9f98-94530d369052")
                IHolographicFrame3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        ABI::Windows::Graphics::Holographic::HolographicFrameId* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFrame3 = __uuidof(IHolographicFrame3);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFramePrediction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFramePrediction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFramePrediction[] = L"Windows.Graphics.Holographic.IHolographicFramePrediction";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("520f4de1-5c0a-4e79-a81e-6abe02bb2739")
                IHolographicFramePrediction : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CameraPoses(
                        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Perception::IPerceptionTimestamp** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFramePrediction = __uuidof(IHolographicFramePrediction);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFramePresentationMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFramePresentationMonitor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFramePresentationMonitor[] = L"Windows.Graphics.Holographic.IHolographicFramePresentationMonitor";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("ca87256c-6fae-428e-bb83-25dfee51136b")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("Use HolographicFrameScanoutMonitor instead of HolographicFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                IHolographicFramePresentationMonitor : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("Use HolographicFrameScanoutMonitor instead of HolographicFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE ReadReports(
                        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFramePresentationMonitor = __uuidof(IHolographicFramePresentationMonitor);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFramePresentationReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFramePresentationReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFramePresentationReport[] = L"Windows.Graphics.Holographic.IHolographicFramePresentationReport";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("80baf614-f2f4-4c8a-8de3-065c78f6d5de")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                IHolographicFramePresentationReport : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_CompositorGpuDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_AppGpuDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_AppGpuOverrun(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_MissedPresentationOpportunityCount(
                        UINT32* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_PresentationCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFramePresentationReport = __uuidof(IHolographicFramePresentationReport);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrameRenderingReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrameRenderingReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrameRenderingReport[] = L"Windows.Graphics.Holographic.IHolographicFrameRenderingReport";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("05f32de4-e384-51b3-b934-f0d3a0f78606")
                IHolographicFrameRenderingReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FrameId(
                        ABI::Windows::Graphics::Holographic::HolographicFrameId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MissedLatchCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemRelativeFrameReadyTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemRelativeActualGpuFinishTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemRelativeTargetLatchTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFrameRenderingReport = __uuidof(IHolographicFrameRenderingReport);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrameScanoutMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrameScanoutMonitor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrameScanoutMonitor[] = L"Windows.Graphics.Holographic.IHolographicFrameScanoutMonitor";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("7e83efa9-843c-5401-8095-9bc1b8b08638")
                IHolographicFrameScanoutMonitor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadReports(
                        __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFrameScanoutMonitor = __uuidof(IHolographicFrameScanoutMonitor);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrameScanoutReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrameScanoutReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrameScanoutReport[] = L"Windows.Graphics.Holographic.IHolographicFrameScanoutReport";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("0ebbe606-03a0-5ca0-b46e-bba068d7233f")
                IHolographicFrameScanoutReport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RenderingReport(
                        ABI::Windows::Graphics::Holographic::IHolographicFrameRenderingReport** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MissedScanoutCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemRelativeLatchTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemRelativeScanoutStartTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemRelativePhotonTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicFrameScanoutReport = __uuidof(IHolographicFrameScanoutReport);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicQuadLayer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicQuadLayer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicQuadLayer[] = L"Windows.Graphics.Holographic.IHolographicQuadLayer";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("903460c9-c9d9-5d5c-41ac-a2d5ab0fd331")
                IHolographicQuadLayer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PixelFormat(
                        ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicQuadLayer = __uuidof(IHolographicQuadLayer);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicQuadLayerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicQuadLayer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicQuadLayerFactory[] = L"Windows.Graphics.Holographic.IHolographicQuadLayerFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("a67538f3-5a14-5a10-489a-455065b37b76")
                IHolographicQuadLayerFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Foundation::Size size,
                        ABI::Windows::Graphics::Holographic::IHolographicQuadLayer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithPixelFormat(
                        ABI::Windows::Foundation::Size size,
                        ABI::Windows::Graphics::DirectX::DirectXPixelFormat pixelFormat,
                        ABI::Windows::Graphics::Holographic::IHolographicQuadLayer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicQuadLayerFactory = __uuidof(IHolographicQuadLayerFactory);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicQuadLayerUpdateParameters[] = L"Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("2b0ea3b0-798d-5bca-55c2-2c0c762ebb08")
                IHolographicQuadLayerUpdateParameters : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AcquireBufferToUpdateContent(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateViewport(
                        ABI::Windows::Foundation::Rect value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateContentProtectionEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateExtents(
                        ABI::Windows::Foundation::Numerics::Vector2 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateLocationWithStationaryMode(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Foundation::Numerics::Vector3 position,
                        ABI::Windows::Foundation::Numerics::Quaternion orientation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateLocationWithDisplayRelativeMode(
                        ABI::Windows::Foundation::Numerics::Vector3 position,
                        ABI::Windows::Foundation::Numerics::Quaternion orientation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicQuadLayerUpdateParameters = __uuidof(IHolographicQuadLayerUpdateParameters);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicQuadLayerUpdateParameters2[] = L"Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("4f33d32d-82c1-46c1-8980-3cb70d98182b")
                IHolographicQuadLayerUpdateParameters2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanAcquireWithHardwareProtection(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AcquireBufferToUpdateContentWithHardwareProtection(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicQuadLayerUpdateParameters2 = __uuidof(IHolographicQuadLayerUpdateParameters2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpace[] = L"Windows.Graphics.Holographic.IHolographicSpace";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("4380dba6-5e78-434f-807c-3433d1efe8b7")
                IHolographicSpace : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PrimaryAdapterId(
                        ABI::Windows::Graphics::Holographic::HolographicAdapterId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDirect3D11Device(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CameraAdded(
                        __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CameraAdded(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CameraRemoved(
                        __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CameraRemoved(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateNextFrame(
                        ABI::Windows::Graphics::Holographic::IHolographicFrame** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicSpace = __uuidof(IHolographicSpace);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpace2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpace2[] = L"Windows.Graphics.Holographic.IHolographicSpace2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("4f81a9a8-b7ff-4883-9827-7d677287ea70")
                IHolographicSpace2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UserPresence(
                        ABI::Windows::Graphics::Holographic::HolographicSpaceUserPresence* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_UserPresenceChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UserPresenceChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WaitForNextFrameReady(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WaitForNextFrameReadyWithHeadStart(
                        ABI::Windows::Foundation::TimeSpan requestedHeadStartDuration
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("Use CreateFrameScanoutMonitor instead of CreateFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE CreateFramePresentationMonitor(
                        UINT32 maxQueuedReports,
                        ABI::Windows::Graphics::Holographic::IHolographicFramePresentationMonitor** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicSpace2 = __uuidof(IHolographicSpace2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpace3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpace3[] = L"Windows.Graphics.Holographic.IHolographicSpace3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("df1733d1-f224-587e-8d71-1e8fc8f07b1f")
                IHolographicSpace3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFrameScanoutMonitor(
                        UINT32 maxQueuedReports,
                        ABI::Windows::Graphics::Holographic::IHolographicFrameScanoutMonitor** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicSpace3 = __uuidof(IHolographicSpace3);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceCameraAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceCameraAddedEventArgs[] = L"Windows.Graphics.Holographic.IHolographicSpaceCameraAddedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("58f1da35-bbb3-3c8f-993d-6c80e7feb99f")
                IHolographicSpaceCameraAddedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Camera(
                        ABI::Windows::Graphics::Holographic::IHolographicCamera** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicSpaceCameraAddedEventArgs = __uuidof(IHolographicSpaceCameraAddedEventArgs);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceCameraRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceCameraRemovedEventArgs[] = L"Windows.Graphics.Holographic.IHolographicSpaceCameraRemovedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("805444a8-f2ae-322e-8da9-836a0a95a4c1")
                IHolographicSpaceCameraRemovedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Camera(
                        ABI::Windows::Graphics::Holographic::IHolographicCamera** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicSpaceCameraRemovedEventArgs = __uuidof(IHolographicSpaceCameraRemovedEventArgs);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceStatics[] = L"Windows.Graphics.Holographic.IHolographicSpaceStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("364e6064-c8f2-3ba1-8391-66b8489e67fd")
                IHolographicSpaceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForCoreWindow(
                        ABI::Windows::UI::Core::ICoreWindow* window,
                        ABI::Windows::Graphics::Holographic::IHolographicSpace** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicSpaceStatics = __uuidof(IHolographicSpaceStatics);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceStatics2[] = L"Windows.Graphics.Holographic.IHolographicSpaceStatics2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("0e777088-75fc-48af-8758-0652f6f07c59")
                IHolographicSpaceStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAvailable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsAvailableChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsAvailableChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicSpaceStatics2 = __uuidof(IHolographicSpaceStatics2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceStatics3[] = L"Windows.Graphics.Holographic.IHolographicSpaceStatics3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("3b00de3d-b1a3-4dfe-8e79-fec5909e6df8")
                IHolographicSpaceStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsConfigured(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicSpaceStatics3 = __uuidof(IHolographicSpaceStatics3);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicViewConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicViewConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicViewConfiguration[] = L"Windows.Graphics.Holographic.IHolographicViewConfiguration";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("5c1de6e6-67e9-5004-b02c-67a3a122b576")
                IHolographicViewConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NativeRenderTargetSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RenderTargetSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestRenderTargetSize(
                        ABI::Windows::Foundation::Size size,
                        ABI::Windows::Foundation::Size* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedPixelFormats(
                        __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PixelFormat(
                        ABI::Windows::Graphics::DirectX::DirectXPixelFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PixelFormat(
                        ABI::Windows::Graphics::DirectX::DirectXPixelFormat value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsStereo(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RefreshRate(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Graphics::Holographic::HolographicViewConfigurationKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Display(
                        ABI::Windows::Graphics::Holographic::IHolographicDisplay** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicViewConfiguration = __uuidof(IHolographicViewConfiguration);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicViewConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicViewConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicViewConfiguration2[] = L"Windows.Graphics.Holographic.IHolographicViewConfiguration2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Holographic {
                MIDL_INTERFACE("e241756e-e0d0-5019-9af5-1b165bc2f54e")
                IHolographicViewConfiguration2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedDepthReprojectionMethods(
                        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicViewConfiguration2 = __uuidof(IHolographicViewConfiguration2);
            } /* Holographic */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicCamera
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicCamera ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicCamera2
 *    Windows.Graphics.Holographic.IHolographicCamera3
 *    Windows.Graphics.Holographic.IHolographicCamera4
 *    Windows.Graphics.Holographic.IHolographicCamera5
 *    Windows.Graphics.Holographic.IHolographicCamera6
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCamera_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCamera_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicCamera[] = L"Windows.Graphics.Holographic.HolographicCamera";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicCameraPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicCameraPose ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicCameraPose2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraPose_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicCameraPose[] = L"Windows.Graphics.Holographic.HolographicCameraPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicCameraRenderingParameters ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2
 *    Windows.Graphics.Holographic.IHolographicCameraRenderingParameters3
 *    Windows.Graphics.Holographic.IHolographicCameraRenderingParameters4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraRenderingParameters_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraRenderingParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicCameraRenderingParameters[] = L"Windows.Graphics.Holographic.HolographicCameraRenderingParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicCameraViewportParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicCameraViewportParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraViewportParameters_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraViewportParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicCameraViewportParameters[] = L"Windows.Graphics.Holographic.HolographicCameraViewportParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicDisplay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Holographic.IHolographicDisplayStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicDisplay ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicDisplay2
 *    Windows.Graphics.Holographic.IHolographicDisplay3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicDisplay_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicDisplay_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicDisplay[] = L"Windows.Graphics.Holographic.HolographicDisplay";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFrame ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicFrame2
 *    Windows.Graphics.Holographic.IHolographicFrame3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrame_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFrame[] = L"Windows.Graphics.Holographic.HolographicFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFramePrediction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFramePrediction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePrediction_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePrediction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFramePrediction[] = L"Windows.Graphics.Holographic.HolographicFramePrediction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFramePresentationMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFramePresentationMonitor ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePresentationMonitor_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePresentationMonitor_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("Use HolographicFrameScanoutMonitor instead of HolographicFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFramePresentationMonitor[] = L"Windows.Graphics.Holographic.HolographicFramePresentationMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFramePresentationReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFramePresentationReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePresentationReport_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePresentationReport_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("Use HolographicFrameScanoutReport instead of HolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFramePresentationReport[] = L"Windows.Graphics.Holographic.HolographicFramePresentationReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFrameRenderingReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFrameRenderingReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameRenderingReport_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameRenderingReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFrameRenderingReport[] = L"Windows.Graphics.Holographic.HolographicFrameRenderingReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFrameScanoutMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFrameScanoutMonitor ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameScanoutMonitor_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameScanoutMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFrameScanoutMonitor[] = L"Windows.Graphics.Holographic.HolographicFrameScanoutMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFrameScanoutReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFrameScanoutReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameScanoutReport_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameScanoutReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFrameScanoutReport[] = L"Windows.Graphics.Holographic.HolographicFrameScanoutReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicQuadLayer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Holographic.IHolographicQuadLayerFactory interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IClosable
 *    Windows.Graphics.Holographic.IHolographicQuadLayer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicQuadLayer_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicQuadLayer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicQuadLayer[] = L"Windows.Graphics.Holographic.HolographicQuadLayer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicQuadLayerUpdateParameters_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicQuadLayerUpdateParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicQuadLayerUpdateParameters[] = L"Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Holographic.IHolographicSpaceStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Holographic.IHolographicSpaceStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Holographic.IHolographicSpaceStatics3 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicSpace ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicSpace2
 *    Windows.Graphics.Holographic.IHolographicSpace3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpace_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpace_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicSpace[] = L"Windows.Graphics.Holographic.HolographicSpace";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicSpaceCameraAddedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpaceCameraAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpaceCameraAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicSpaceCameraAddedEventArgs[] = L"Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicSpaceCameraRemovedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpaceCameraRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpaceCameraRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicSpaceCameraRemovedEventArgs[] = L"Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicViewConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicViewConfiguration ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicViewConfiguration2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicViewConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicViewConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicViewConfiguration[] = L"Windows.Graphics.Holographic.HolographicViewConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2 __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2;

#endif // ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

typedef struct __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl;

interface __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

typedef struct __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        __FIIterator_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl;

interface __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera;

typedef struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl;

interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera;

typedef struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCamera** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl;

interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose;

typedef struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl;

interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose;

typedef struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicCameraPose** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl;

interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod;

typedef struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl;

interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod;

typedef struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl;

interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport;

typedef struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl;

interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport;

typedef struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl;

interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport;

typedef struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl;

interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport;

typedef struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl;

interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer;

typedef struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl;

interface __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer;

typedef struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        __FIIterator_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl;

interface __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat;

typedef struct __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        UINT32 index,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl;

interface __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera;

typedef struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl;

interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose;

typedef struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl;

interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod;

typedef struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        UINT32 index,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl;

interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethodVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport;

typedef struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl;

interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport;

typedef struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl;

interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer;

typedef struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl;

interface __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport;

typedef struct __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl;

interface __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer;

typedef struct __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl;

interface __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicStereoTransform __x_ABI_CWindows_CGraphics_CHolographic_CHolographicStereoTransform;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform;

typedef struct __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransformVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform* This,
        struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicStereoTransform* result);

    END_INTERFACE
} __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransformVtbl;

interface __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform
{
    CONST_VTBL struct __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransformVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingFrustum __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingFrustum;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum;

typedef struct __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustumVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum* This,
        struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialBoundingFrustum* result);

    END_INTERFACE
} __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustumVtbl;

interface __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum
{
    CONST_VTBL struct __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustumVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* sender,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* sender,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4 __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 __x_ABI_CWindows_CFoundation_CNumerics_CVector2;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CIPerceptionTimestamp __x_ABI_CWindows_CPerception_CIPerceptionTimestamp;

#endif // ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindow __x_ABI_CWindows_CUI_CCore_CICoreWindow;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentResult __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentResult;

typedef enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentWaitBehavior __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentWaitBehavior;

typedef enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicReprojectionMode __x_ABI_CWindows_CGraphics_CHolographic_CHolographicReprojectionMode;

typedef enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicSpaceUserPresence __x_ABI_CWindows_CGraphics_CHolographic_CHolographicSpaceUserPresence;

typedef enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicViewConfigurationKind __x_ABI_CWindows_CGraphics_CHolographic_CHolographicViewConfigurationKind;

typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicAdapterId __x_ABI_CWindows_CGraphics_CHolographic_CHolographicAdapterId;

typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFrameId __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFrameId;

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicDepthReprojectionMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod
{
    HolographicDepthReprojectionMethod_DepthReprojection = 0,
    HolographicDepthReprojectionMethod_AutoPlanar = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicFramePresentResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentResult
{
    HolographicFramePresentResult_Success = 0,
    HolographicFramePresentResult_DeviceRemoved = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicFramePresentWaitBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentWaitBehavior
{
    HolographicFramePresentWaitBehavior_WaitForFrameToFinish = 0,
    HolographicFramePresentWaitBehavior_DoNotWaitForFrameToFinish = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicReprojectionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicReprojectionMode
{
    HolographicReprojectionMode_PositionAndOrientation = 0,
    HolographicReprojectionMode_OrientationOnly = 1,
    HolographicReprojectionMode_Disabled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicSpaceUserPresence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicSpaceUserPresence
{
    HolographicSpaceUserPresence_Absent = 0,
    HolographicSpaceUserPresence_PresentPassive = 1,
    HolographicSpaceUserPresence_PresentActive = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicViewConfigurationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicViewConfigurationKind
{
    HolographicViewConfigurationKind_Display = 0,
    HolographicViewConfigurationKind_PhotoVideoCamera = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicAdapterId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicAdapterId
{
    UINT32 LowPart;
    INT32 HighPart;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicFrameId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFrameId
{
    UINT64 Value;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.Graphics.Holographic.HolographicStereoTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicStereoTransform
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4 Left;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4 Right;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera[] = L"Windows.Graphics.Holographic.IHolographicCamera";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RenderTargetSize)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_ViewportScaleFactor)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_ViewportScaleFactor)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_IsStereo)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SetNearPlaneDistance)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* SetFarPlaneDistance)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_get_RenderTargetSize(This, value) \
    ((This)->lpVtbl->get_RenderTargetSize(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_get_ViewportScaleFactor(This, value) \
    ((This)->lpVtbl->get_ViewportScaleFactor(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_put_ViewportScaleFactor(This, value) \
    ((This)->lpVtbl->put_ViewportScaleFactor(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_get_IsStereo(This, value) \
    ((This)->lpVtbl->get_IsStereo(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_SetNearPlaneDistance(This, value) \
    ((This)->lpVtbl->SetNearPlaneDistance(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_SetFarPlaneDistance(This, value) \
    ((This)->lpVtbl->SetFarPlaneDistance(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera2[] = L"Windows.Graphics.Holographic.IHolographicCamera2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LeftViewportParameters)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters** result);
    HRESULT (STDMETHODCALLTYPE* get_RightViewportParameters)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters** result);
    HRESULT (STDMETHODCALLTYPE* get_Display)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_get_LeftViewportParameters(This, result) \
    ((This)->lpVtbl->get_LeftViewportParameters(This, result))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_get_RightViewportParameters(This, result) \
    ((This)->lpVtbl->get_RightViewportParameters(This, result))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_get_Display(This, result) \
    ((This)->lpVtbl->get_Display(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicCamera2
 *     Windows.Graphics.Holographic.IHolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera3[] = L"Windows.Graphics.Holographic.IHolographicCamera3";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPrimaryLayerEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsPrimaryLayerEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MaxQuadLayerCount)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_QuadLayers)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3* This,
        __FIVector_1_Windows__CGraphics__CHolographic__CHolographicQuadLayer** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_get_IsPrimaryLayerEnabled(This, value) \
    ((This)->lpVtbl->get_IsPrimaryLayerEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_put_IsPrimaryLayerEnabled(This, value) \
    ((This)->lpVtbl->put_IsPrimaryLayerEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_get_MaxQuadLayerCount(This, value) \
    ((This)->lpVtbl->get_MaxQuadLayerCount(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_get_QuadLayers(This, value) \
    ((This)->lpVtbl->get_QuadLayers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera4[] = L"Windows.Graphics.Holographic.IHolographicCamera4";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanOverrideViewport)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_get_CanOverrideViewport(This, value) \
    ((This)->lpVtbl->get_CanOverrideViewport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera5[] = L"Windows.Graphics.Holographic.IHolographicCamera5";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsHardwareContentProtectionSupported)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsHardwareContentProtectionEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsHardwareContentProtectionEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_get_IsHardwareContentProtectionSupported(This, value) \
    ((This)->lpVtbl->get_IsHardwareContentProtectionSupported(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_get_IsHardwareContentProtectionEnabled(This, value) \
    ((This)->lpVtbl->get_IsHardwareContentProtectionEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_put_IsHardwareContentProtectionEnabled(This, value) \
    ((This)->lpVtbl->put_IsHardwareContentProtectionEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCamera6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCamera
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCamera6[] = L"Windows.Graphics.Holographic.IHolographicCamera6";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewConfiguration)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_get_ViewConfiguration(This, value) \
    ((This)->lpVtbl->get_ViewConfiguration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraPose[] = L"Windows.Graphics.Holographic.IHolographicCameraPose";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HolographicCamera)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera** value);
    HRESULT (STDMETHODCALLTYPE* get_Viewport)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* TryGetViewTransform)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __FIReference_1_Windows__CGraphics__CHolographic__CHolographicStereoTransform** value);
    HRESULT (STDMETHODCALLTYPE* get_ProjectionTransform)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicStereoTransform* value);
    HRESULT (STDMETHODCALLTYPE* TryGetCullingFrustum)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum** value);
    HRESULT (STDMETHODCALLTYPE* TryGetVisibleFrustum)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __FIReference_1_Windows__CPerception__CSpatial__CSpatialBoundingFrustum** value);
    HRESULT (STDMETHODCALLTYPE* get_NearPlaneDistance)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_FarPlaneDistance)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPoseVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_get_HolographicCamera(This, value) \
    ((This)->lpVtbl->get_HolographicCamera(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_get_Viewport(This, value) \
    ((This)->lpVtbl->get_Viewport(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_TryGetViewTransform(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetViewTransform(This, coordinateSystem, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_get_ProjectionTransform(This, value) \
    ((This)->lpVtbl->get_ProjectionTransform(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_TryGetCullingFrustum(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetCullingFrustum(This, coordinateSystem, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_TryGetVisibleFrustum(This, coordinateSystem, value) \
    ((This)->lpVtbl->TryGetVisibleFrustum(This, coordinateSystem, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_get_NearPlaneDistance(This, value) \
    ((This)->lpVtbl->get_NearPlaneDistance(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_get_FarPlaneDistance(This, value) \
    ((This)->lpVtbl->get_FarPlaneDistance(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraPose2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraPose2[] = L"Windows.Graphics.Holographic.IHolographicCameraPose2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OverrideViewTransform)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicStereoTransform coordinateSystemToViewTransform);
    HRESULT (STDMETHODCALLTYPE* OverrideProjectionTransform)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This,
        struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicStereoTransform projectionTransform);
    HRESULT (STDMETHODCALLTYPE* OverrideViewport)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2* This,
        struct __x_ABI_CWindows_CFoundation_CRect leftViewport,
        struct __x_ABI_CWindows_CFoundation_CRect rightViewport);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_OverrideViewTransform(This, coordinateSystem, coordinateSystemToViewTransform) \
    ((This)->lpVtbl->OverrideViewTransform(This, coordinateSystem, coordinateSystemToViewTransform))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_OverrideProjectionTransform(This, projectionTransform) \
    ((This)->lpVtbl->OverrideProjectionTransform(This, projectionTransform))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_OverrideViewport(This, leftViewport, rightViewport) \
    ((This)->lpVtbl->OverrideViewport(This, leftViewport, rightViewport))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraRenderingParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraRenderingParameters[] = L"Windows.Graphics.Holographic.IHolographicCameraRenderingParameters";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetFocusPoint)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 position);
    HRESULT (STDMETHODCALLTYPE* SetFocusPointWithNormal)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 position,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 normal);
    HRESULT (STDMETHODCALLTYPE* SetFocusPointWithNormalLinearVelocity)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 position,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 normal,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 linearVelocity);
    HRESULT (STDMETHODCALLTYPE* get_Direct3D11Device)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice** value);
    HRESULT (STDMETHODCALLTYPE* get_Direct3D11BackBuffer)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParametersVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_SetFocusPoint(This, coordinateSystem, position) \
    ((This)->lpVtbl->SetFocusPoint(This, coordinateSystem, position))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_SetFocusPointWithNormal(This, coordinateSystem, position, normal) \
    ((This)->lpVtbl->SetFocusPointWithNormal(This, coordinateSystem, position, normal))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_SetFocusPointWithNormalLinearVelocity(This, coordinateSystem, position, normal, linearVelocity) \
    ((This)->lpVtbl->SetFocusPointWithNormalLinearVelocity(This, coordinateSystem, position, normal, linearVelocity))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_get_Direct3D11Device(This, value) \
    ((This)->lpVtbl->get_Direct3D11Device(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_get_Direct3D11BackBuffer(This, value) \
    ((This)->lpVtbl->get_Direct3D11BackBuffer(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicCameraRenderingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraRenderingParameters2[] = L"Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ReprojectionMode)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicReprojectionMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ReprojectionMode)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicReprojectionMode value);
    HRESULT (STDMETHODCALLTYPE* CommitDirect3D11DepthBuffer)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_get_ReprojectionMode(This, value) \
    ((This)->lpVtbl->get_ReprojectionMode(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_put_ReprojectionMode(This, value) \
    ((This)->lpVtbl->put_ReprojectionMode(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_CommitDirect3D11DepthBuffer(This, value) \
    ((This)->lpVtbl->CommitDirect3D11DepthBuffer(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraRenderingParameters3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2
 *     Windows.Graphics.Holographic.IHolographicCameraRenderingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraRenderingParameters3[] = L"Windows.Graphics.Holographic.IHolographicCameraRenderingParameters3";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsContentProtectionEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsContentProtectionEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_get_IsContentProtectionEnabled(This, value) \
    ((This)->lpVtbl->get_IsContentProtectionEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_put_IsContentProtectionEnabled(This, value) \
    ((This)->lpVtbl->put_IsContentProtectionEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraRenderingParameters4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraRenderingParameters4[] = L"Windows.Graphics.Holographic.IHolographicCameraRenderingParameters4";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DepthReprojectionMethod)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod* value);
    HRESULT (STDMETHODCALLTYPE* put_DepthReprojectionMethod)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicDepthReprojectionMethod value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_get_DepthReprojectionMethod(This, value) \
    ((This)->lpVtbl->get_DepthReprojectionMethod(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_put_DepthReprojectionMethod(This, value) \
    ((This)->lpVtbl->put_DepthReprojectionMethod(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicCameraViewportParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicCameraViewportParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicCameraViewportParameters[] = L"Windows.Graphics.Holographic.IHolographicCameraViewportParameters";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HiddenAreaMesh)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters* This,
        UINT32* valueLength,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2** value);
    HRESULT (STDMETHODCALLTYPE* get_VisibleAreaMesh)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters* This,
        UINT32* valueLength,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParametersVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_get_HiddenAreaMesh(This, valueLength, value) \
    ((This)->lpVtbl->get_HiddenAreaMesh(This, valueLength, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_get_VisibleAreaMesh(This, valueLength, value) \
    ((This)->lpVtbl->get_VisibleAreaMesh(This, valueLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraViewportParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicDisplay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicDisplay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicDisplay[] = L"Windows.Graphics.Holographic.IHolographicDisplay";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxViewportSize)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_IsStereo)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOpaque)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AdapterId)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicAdapterId* value);
    HRESULT (STDMETHODCALLTYPE* get_SpatialLocator)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialLocator** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_get_MaxViewportSize(This, value) \
    ((This)->lpVtbl->get_MaxViewportSize(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_get_IsStereo(This, value) \
    ((This)->lpVtbl->get_IsStereo(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_get_IsOpaque(This, value) \
    ((This)->lpVtbl->get_IsOpaque(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_get_AdapterId(This, value) \
    ((This)->lpVtbl->get_AdapterId(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_get_SpatialLocator(This, value) \
    ((This)->lpVtbl->get_SpatialLocator(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicDisplay2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicDisplay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicDisplay2[] = L"Windows.Graphics.Holographic.IHolographicDisplay2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RefreshRate)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_get_RefreshRate(This, value) \
    ((This)->lpVtbl->get_RefreshRate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicDisplay3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicDisplay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicDisplay3[] = L"Windows.Graphics.Holographic.IHolographicDisplay3";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetViewConfiguration)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicViewConfigurationKind kind,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_TryGetViewConfiguration(This, kind, result) \
    ((This)->lpVtbl->TryGetViewConfiguration(This, kind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicDisplayStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicDisplay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicDisplayStatics[] = L"Windows.Graphics.Holographic.IHolographicDisplayStatics";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplayStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrame[] = L"Windows.Graphics.Holographic.IHolographicFrame";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AddedCameras)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera** value);
    HRESULT (STDMETHODCALLTYPE* get_RemovedCameras)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCamera** value);
    HRESULT (STDMETHODCALLTYPE* GetRenderingParameters)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraPose* cameraPose,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCameraRenderingParameters** value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentPrediction)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction** value);
    HRESULT (STDMETHODCALLTYPE* UpdateCurrentPrediction)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This);
    HRESULT (STDMETHODCALLTYPE* PresentUsingCurrentPrediction)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentResult* result);
    HRESULT (STDMETHODCALLTYPE* PresentUsingCurrentPredictionWithBehavior)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentWaitBehavior waitBehavior,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFramePresentResult* result);
    HRESULT (STDMETHODCALLTYPE* WaitForFrameToFinish)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame* This);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_get_AddedCameras(This, value) \
    ((This)->lpVtbl->get_AddedCameras(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_get_RemovedCameras(This, value) \
    ((This)->lpVtbl->get_RemovedCameras(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_GetRenderingParameters(This, cameraPose, value) \
    ((This)->lpVtbl->GetRenderingParameters(This, cameraPose, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_get_CurrentPrediction(This, value) \
    ((This)->lpVtbl->get_CurrentPrediction(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_UpdateCurrentPrediction(This) \
    ((This)->lpVtbl->UpdateCurrentPrediction(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_PresentUsingCurrentPrediction(This, result) \
    ((This)->lpVtbl->PresentUsingCurrentPrediction(This, result))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_PresentUsingCurrentPredictionWithBehavior(This, waitBehavior, result) \
    ((This)->lpVtbl->PresentUsingCurrentPredictionWithBehavior(This, waitBehavior, result))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_WaitForFrameToFinish(This) \
    ((This)->lpVtbl->WaitForFrameToFinish(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrame2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrame
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Holographic.IHolographicFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrame2[] = L"Windows.Graphics.Holographic.IHolographicFrame2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetQuadLayerUpdateParameters)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* layer,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_GetQuadLayerUpdateParameters(This, layer, value) \
    ((This)->lpVtbl->GetQuadLayerUpdateParameters(This, layer, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrame3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrame3[] = L"Windows.Graphics.Holographic.IHolographicFrame3";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3* This,
        struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFrameId* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFramePrediction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFramePrediction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFramePrediction[] = L"Windows.Graphics.Holographic.IHolographicFramePrediction";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePredictionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CameraPoses)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction* This,
        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicCameraPose** value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePredictionVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePredictionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_get_CameraPoses(This, value) \
    ((This)->lpVtbl->get_CameraPoses(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePrediction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFramePresentationMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFramePresentationMonitor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFramePresentationMonitor[] = L"Windows.Graphics.Holographic.IHolographicFramePresentationMonitor";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("Use HolographicFrameScanoutMonitor instead of HolographicFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use HolographicFrameScanoutMonitor instead of HolographicFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ReadReports)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor* This,
        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicFramePresentationReport** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitorVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use HolographicFrameScanoutMonitor instead of HolographicFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_ReadReports(This, result) \
    ((This)->lpVtbl->ReadReports(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFramePresentationReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFramePresentationReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFramePresentationReport[] = L"Windows.Graphics.Holographic.IHolographicFramePresentationReport";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_CompositorGpuDuration)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_AppGpuDuration)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_AppGpuOverrun)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_MissedPresentationOpportunityCount)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        UINT32* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_PresentationCount)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReportVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_get_CompositorGpuDuration(This, value) \
    ((This)->lpVtbl->get_CompositorGpuDuration(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_get_AppGpuDuration(This, value) \
    ((This)->lpVtbl->get_AppGpuDuration(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_get_AppGpuOverrun(This, value) \
    ((This)->lpVtbl->get_AppGpuOverrun(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_get_MissedPresentationOpportunityCount(This, value) \
    ((This)->lpVtbl->get_MissedPresentationOpportunityCount(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use IHolographicFrameScanoutReport instead of IHolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_get_PresentationCount(This, value) \
    ((This)->lpVtbl->get_PresentationCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrameRenderingReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrameRenderingReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrameRenderingReport[] = L"Windows.Graphics.Holographic.IHolographicFrameRenderingReport";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FrameId)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicFrameId* value);
    HRESULT (STDMETHODCALLTYPE* get_MissedLatchCount)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativeFrameReadyTime)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativeActualGpuFinishTime)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativeTargetLatchTime)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReportVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_get_FrameId(This, value) \
    ((This)->lpVtbl->get_FrameId(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_get_MissedLatchCount(This, value) \
    ((This)->lpVtbl->get_MissedLatchCount(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_get_SystemRelativeFrameReadyTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativeFrameReadyTime(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_get_SystemRelativeActualGpuFinishTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativeActualGpuFinishTime(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_get_SystemRelativeTargetLatchTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativeTargetLatchTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrameScanoutMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrameScanoutMonitor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrameScanoutMonitor[] = L"Windows.Graphics.Holographic.IHolographicFrameScanoutMonitor";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadReports)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor* This,
        __FIVector_1_Windows__CGraphics__CHolographic__CHolographicFrameScanoutReport** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitorVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_ReadReports(This, result) \
    ((This)->lpVtbl->ReadReports(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicFrameScanoutReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicFrameScanoutReport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicFrameScanoutReport[] = L"Windows.Graphics.Holographic.IHolographicFrameScanoutReport";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RenderingReport)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameRenderingReport** value);
    HRESULT (STDMETHODCALLTYPE* get_MissedScanoutCount)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativeLatchTime)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativeScanoutStartTime)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativePhotonTime)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReportVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_get_RenderingReport(This, value) \
    ((This)->lpVtbl->get_RenderingReport(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_get_MissedScanoutCount(This, value) \
    ((This)->lpVtbl->get_MissedScanoutCount(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_get_SystemRelativeLatchTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativeLatchTime(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_get_SystemRelativeScanoutStartTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativeScanoutStartTime(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_get_SystemRelativePhotonTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativePhotonTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutReport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicQuadLayer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicQuadLayer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicQuadLayer[] = L"Windows.Graphics.Holographic.IHolographicQuadLayer";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PixelFormat)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_get_PixelFormat(This, value) \
    ((This)->lpVtbl->get_PixelFormat(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicQuadLayerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicQuadLayer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicQuadLayerFactory[] = L"Windows.Graphics.Holographic.IHolographicQuadLayerFactory";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory* This,
        struct __x_ABI_CWindows_CFoundation_CSize size,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithPixelFormat)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory* This,
        struct __x_ABI_CWindows_CFoundation_CSize size,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat pixelFormat,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayer** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_Create(This, size, value) \
    ((This)->lpVtbl->Create(This, size, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_CreateWithPixelFormat(This, size, pixelFormat, value) \
    ((This)->lpVtbl->CreateWithPixelFormat(This, size, pixelFormat, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicQuadLayerUpdateParameters[] = L"Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AcquireBufferToUpdateContent)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** value);
    HRESULT (STDMETHODCALLTYPE* UpdateViewport)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        struct __x_ABI_CWindows_CFoundation_CRect value);
    HRESULT (STDMETHODCALLTYPE* UpdateContentProtectionEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* UpdateExtents)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 value);
    HRESULT (STDMETHODCALLTYPE* UpdateLocationWithStationaryMode)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 position,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion orientation);
    HRESULT (STDMETHODCALLTYPE* UpdateLocationWithDisplayRelativeMode)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 position,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion orientation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParametersVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_AcquireBufferToUpdateContent(This, value) \
    ((This)->lpVtbl->AcquireBufferToUpdateContent(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_UpdateViewport(This, value) \
    ((This)->lpVtbl->UpdateViewport(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_UpdateContentProtectionEnabled(This, value) \
    ((This)->lpVtbl->UpdateContentProtectionEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_UpdateExtents(This, value) \
    ((This)->lpVtbl->UpdateExtents(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_UpdateLocationWithStationaryMode(This, coordinateSystem, position, orientation) \
    ((This)->lpVtbl->UpdateLocationWithStationaryMode(This, coordinateSystem, position, orientation))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_UpdateLocationWithDisplayRelativeMode(This, position, orientation) \
    ((This)->lpVtbl->UpdateLocationWithDisplayRelativeMode(This, position, orientation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicQuadLayerUpdateParameters2[] = L"Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanAcquireWithHardwareProtection)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* AcquireBufferToUpdateContentWithHardwareProtection)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_get_CanAcquireWithHardwareProtection(This, value) \
    ((This)->lpVtbl->get_CanAcquireWithHardwareProtection(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_AcquireBufferToUpdateContentWithHardwareProtection(This, value) \
    ((This)->lpVtbl->AcquireBufferToUpdateContentWithHardwareProtection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicQuadLayerUpdateParameters2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpace[] = L"Windows.Graphics.Holographic.IHolographicSpace";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrimaryAdapterId)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        struct __x_ABI_CWindows_CGraphics_CHolographic_CHolographicAdapterId* value);
    HRESULT (STDMETHODCALLTYPE* SetDirect3D11Device)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* value);
    HRESULT (STDMETHODCALLTYPE* add_CameraAdded)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraAddedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_CameraAdded)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_CameraRemoved)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_Windows__CGraphics__CHolographic__CHolographicSpaceCameraRemovedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_CameraRemoved)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* CreateNextFrame)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_get_PrimaryAdapterId(This, value) \
    ((This)->lpVtbl->get_PrimaryAdapterId(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_SetDirect3D11Device(This, value) \
    ((This)->lpVtbl->SetDirect3D11Device(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_add_CameraAdded(This, handler, cookie) \
    ((This)->lpVtbl->add_CameraAdded(This, handler, cookie))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_remove_CameraAdded(This, cookie) \
    ((This)->lpVtbl->remove_CameraAdded(This, cookie))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_add_CameraRemoved(This, handler, cookie) \
    ((This)->lpVtbl->add_CameraRemoved(This, handler, cookie))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_remove_CameraRemoved(This, cookie) \
    ((This)->lpVtbl->remove_CameraRemoved(This, cookie))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_CreateNextFrame(This, value) \
    ((This)->lpVtbl->CreateNextFrame(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpace2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpace2[] = L"Windows.Graphics.Holographic.IHolographicSpace2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserPresence)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicSpaceUserPresence* value);
    HRESULT (STDMETHODCALLTYPE* add_UserPresenceChanged)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        __FITypedEventHandler_2_Windows__CGraphics__CHolographic__CHolographicSpace_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UserPresenceChanged)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* WaitForNextFrameReady)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This);
    HRESULT (STDMETHODCALLTYPE* WaitForNextFrameReadyWithHeadStart)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan requestedHeadStartDuration);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use CreateFrameScanoutMonitor instead of CreateFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* CreateFramePresentationMonitor)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2* This,
        UINT32 maxQueuedReports,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFramePresentationMonitor** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_get_UserPresence(This, value) \
    ((This)->lpVtbl->get_UserPresence(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_add_UserPresenceChanged(This, handler, token) \
    ((This)->lpVtbl->add_UserPresenceChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_remove_UserPresenceChanged(This, token) \
    ((This)->lpVtbl->remove_UserPresenceChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_WaitForNextFrameReady(This) \
    ((This)->lpVtbl->WaitForNextFrameReady(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_WaitForNextFrameReadyWithHeadStart(This, requestedHeadStartDuration) \
    ((This)->lpVtbl->WaitForNextFrameReadyWithHeadStart(This, requestedHeadStartDuration))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("Use CreateFrameScanoutMonitor instead of CreateFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_CreateFramePresentationMonitor(This, maxQueuedReports, result) \
    ((This)->lpVtbl->CreateFramePresentationMonitor(This, maxQueuedReports, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpace3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpace3[] = L"Windows.Graphics.Holographic.IHolographicSpace3";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFrameScanoutMonitor)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3* This,
        UINT32 maxQueuedReports,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicFrameScanoutMonitor** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_CreateFrameScanoutMonitor(This, maxQueuedReports, result) \
    ((This)->lpVtbl->CreateFrameScanoutMonitor(This, maxQueuedReports, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceCameraAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceCameraAddedEventArgs[] = L"Windows.Graphics.Holographic.IHolographicSpaceCameraAddedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Camera)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_get_Camera(This, value) \
    ((This)->lpVtbl->get_Camera(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceCameraRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceCameraRemovedEventArgs[] = L"Windows.Graphics.Holographic.IHolographicSpaceCameraRemovedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Camera)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicCamera** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_get_Camera(This, value) \
    ((This)->lpVtbl->get_Camera(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceCameraRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceStatics[] = L"Windows.Graphics.Holographic.IHolographicSpaceStatics";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForCoreWindow)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* window,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpace** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_CreateForCoreWindow(This, window, value) \
    ((This)->lpVtbl->CreateForCoreWindow(This, window, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceStatics2[] = L"Windows.Graphics.Holographic.IHolographicSpaceStatics2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAvailable)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_IsAvailableChanged)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsAvailableChanged)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_get_IsAvailable(This, value) \
    ((This)->lpVtbl->get_IsAvailable(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_add_IsAvailableChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsAvailableChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_remove_IsAvailableChanged(This, token) \
    ((This)->lpVtbl->remove_IsAvailableChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicSpaceStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicSpace
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicSpaceStatics3[] = L"Windows.Graphics.Holographic.IHolographicSpaceStatics3";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsConfigured)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_get_IsConfigured(This, value) \
    ((This)->lpVtbl->get_IsConfigured(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicSpaceStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicViewConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicViewConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicViewConfiguration[] = L"Windows.Graphics.Holographic.IHolographicViewConfiguration";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NativeRenderTargetSize)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_RenderTargetSize)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* RequestRenderTargetSize)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        struct __x_ABI_CWindows_CFoundation_CSize size,
        struct __x_ABI_CWindows_CFoundation_CSize* result);
    HRESULT (STDMETHODCALLTYPE* get_SupportedPixelFormats)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        __FIVectorView_1_Windows__CGraphics__CDirectX__CDirectXPixelFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_PixelFormat)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_PixelFormat)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat value);
    HRESULT (STDMETHODCALLTYPE* get_IsStereo)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_RefreshRate)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CHolographic_CHolographicViewConfigurationKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Display)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicDisplay** value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfigurationVtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_NativeRenderTargetSize(This, value) \
    ((This)->lpVtbl->get_NativeRenderTargetSize(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_RenderTargetSize(This, value) \
    ((This)->lpVtbl->get_RenderTargetSize(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_RequestRenderTargetSize(This, size, result) \
    ((This)->lpVtbl->RequestRenderTargetSize(This, size, result))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_SupportedPixelFormats(This, value) \
    ((This)->lpVtbl->get_SupportedPixelFormats(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_PixelFormat(This, value) \
    ((This)->lpVtbl->get_PixelFormat(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_put_PixelFormat(This, value) \
    ((This)->lpVtbl->put_PixelFormat(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_IsStereo(This, value) \
    ((This)->lpVtbl->get_IsStereo(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_RefreshRate(This, value) \
    ((This)->lpVtbl->get_RefreshRate(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_Display(This, value) \
    ((This)->lpVtbl->get_Display(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Graphics.Holographic.IHolographicViewConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Holographic.HolographicViewConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Holographic_IHolographicViewConfiguration2[] = L"Windows.Graphics.Holographic.IHolographicViewConfiguration2";
typedef struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportedDepthReprojectionMethods)(__x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2* This,
        __FIVectorView_1_Windows__CGraphics__CHolographic__CHolographicDepthReprojectionMethod** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2Vtbl;

interface __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_get_SupportedDepthReprojectionMethods(This, value) \
    ((This)->lpVtbl->get_SupportedDepthReprojectionMethods(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CHolographic_CIHolographicViewConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicCamera
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicCamera ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicCamera2
 *    Windows.Graphics.Holographic.IHolographicCamera3
 *    Windows.Graphics.Holographic.IHolographicCamera4
 *    Windows.Graphics.Holographic.IHolographicCamera5
 *    Windows.Graphics.Holographic.IHolographicCamera6
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCamera_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCamera_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicCamera[] = L"Windows.Graphics.Holographic.HolographicCamera";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicCameraPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicCameraPose ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicCameraPose2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraPose_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicCameraPose[] = L"Windows.Graphics.Holographic.HolographicCameraPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicCameraRenderingParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicCameraRenderingParameters ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2
 *    Windows.Graphics.Holographic.IHolographicCameraRenderingParameters3
 *    Windows.Graphics.Holographic.IHolographicCameraRenderingParameters4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraRenderingParameters_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraRenderingParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicCameraRenderingParameters[] = L"Windows.Graphics.Holographic.HolographicCameraRenderingParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicCameraViewportParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicCameraViewportParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraViewportParameters_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicCameraViewportParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicCameraViewportParameters[] = L"Windows.Graphics.Holographic.HolographicCameraViewportParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicDisplay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Holographic.IHolographicDisplayStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicDisplay ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicDisplay2
 *    Windows.Graphics.Holographic.IHolographicDisplay3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicDisplay_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicDisplay_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicDisplay[] = L"Windows.Graphics.Holographic.HolographicDisplay";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFrame ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicFrame2
 *    Windows.Graphics.Holographic.IHolographicFrame3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrame_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFrame[] = L"Windows.Graphics.Holographic.HolographicFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFramePrediction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFramePrediction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePrediction_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePrediction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFramePrediction[] = L"Windows.Graphics.Holographic.HolographicFramePrediction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFramePresentationMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFramePresentationMonitor ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePresentationMonitor_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePresentationMonitor_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("Use HolographicFrameScanoutMonitor instead of HolographicFramePresentationMonitor. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFramePresentationMonitor[] = L"Windows.Graphics.Holographic.HolographicFramePresentationMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFramePresentationReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFramePresentationReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePresentationReport_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFramePresentationReport_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("Use HolographicFrameScanoutReport instead of HolographicFramePresentationReport. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFramePresentationReport[] = L"Windows.Graphics.Holographic.HolographicFramePresentationReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFrameRenderingReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFrameRenderingReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameRenderingReport_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameRenderingReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFrameRenderingReport[] = L"Windows.Graphics.Holographic.HolographicFrameRenderingReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFrameScanoutMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFrameScanoutMonitor ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameScanoutMonitor_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameScanoutMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFrameScanoutMonitor[] = L"Windows.Graphics.Holographic.HolographicFrameScanoutMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicFrameScanoutReport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicFrameScanoutReport ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameScanoutReport_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicFrameScanoutReport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicFrameScanoutReport[] = L"Windows.Graphics.Holographic.HolographicFrameScanoutReport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicQuadLayer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Holographic.IHolographicQuadLayerFactory interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IClosable
 *    Windows.Graphics.Holographic.IHolographicQuadLayer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicQuadLayer_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicQuadLayer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicQuadLayer[] = L"Windows.Graphics.Holographic.HolographicQuadLayer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicQuadLayerUpdateParameters_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicQuadLayerUpdateParameters_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicQuadLayerUpdateParameters[] = L"Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Holographic.IHolographicSpaceStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Holographic.IHolographicSpaceStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Graphics.Holographic.IHolographicSpaceStatics3 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicSpace ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicSpace2
 *    Windows.Graphics.Holographic.IHolographicSpace3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpace_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpace_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicSpace[] = L"Windows.Graphics.Holographic.HolographicSpace";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicSpaceCameraAddedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpaceCameraAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpaceCameraAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicSpaceCameraAddedEventArgs[] = L"Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicSpaceCameraRemovedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpaceCameraRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicSpaceCameraRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicSpaceCameraRemovedEventArgs[] = L"Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Graphics.Holographic.HolographicViewConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Holographic.IHolographicViewConfiguration ** Default Interface **
 *    Windows.Graphics.Holographic.IHolographicViewConfiguration2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Graphics_Holographic_HolographicViewConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Holographic_HolographicViewConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Holographic_HolographicViewConfiguration[] = L"Windows.Graphics.Holographic.HolographicViewConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egraphics2Eholographic_p_h__

#endif // __windows2Egraphics2Eholographic_h__
