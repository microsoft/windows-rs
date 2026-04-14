
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
#ifndef __windows2Egraphics2Eprinting3d_h__
#define __windows2Egraphics2Eprinting3d_h__
#ifndef __windows2Egraphics2Eprinting3d_p_h__
#define __windows2Egraphics2Eprinting3d_p_h__


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

#if !defined(WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION)
#define WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DTaskSourceRequestedHandler;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler ABI::Windows::Graphics::Printing3D::IPrint3DTaskSourceRequestedHandler

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DManager;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager ABI::Windows::Graphics::Printing3D::IPrint3DManager

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DManagerStatics;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics ABI::Windows::Graphics::Printing3D::IPrint3DManagerStatics

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DTask;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask ABI::Windows::Graphics::Printing3D::IPrint3DTask

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DTaskCompletedEventArgs;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs ABI::Windows::Graphics::Printing3D::IPrint3DTaskCompletedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DTaskRequest;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest ABI::Windows::Graphics::Printing3D::IPrint3DTaskRequest

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DTaskRequestedEventArgs;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs ABI::Windows::Graphics::Printing3D::IPrint3DTaskRequestedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DTaskSourceChangedEventArgs;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs ABI::Windows::Graphics::Printing3D::IPrint3DTaskSourceChangedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrint3DTaskSourceRequestedArgs;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs ABI::Windows::Graphics::Printing3D::IPrint3DTaskSourceRequestedArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3D3MFPackage;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage ABI::Windows::Graphics::Printing3D::IPrinting3D3MFPackage

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3D3MFPackage2;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2 ABI::Windows::Graphics::Printing3D::IPrinting3D3MFPackage2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3D3MFPackageStatics;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics ABI::Windows::Graphics::Printing3D::IPrinting3D3MFPackageStatics

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DBaseMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterial

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DBaseMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroup

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DBaseMaterialGroupFactory;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroupFactory

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DBaseMaterialStatics;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialStatics

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DColorMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterial

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DColorMaterial2;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2 ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterial2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DColorMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterialGroup

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DColorMaterialGroupFactory;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterialGroupFactory

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DComponent;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent ABI::Windows::Graphics::Printing3D::IPrinting3DComponent

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DComponentWithMatrix;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix ABI::Windows::Graphics::Printing3D::IPrinting3DComponentWithMatrix

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DCompositeMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterial

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DCompositeMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterialGroup

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DCompositeMaterialGroup2;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2 ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterialGroup2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DCompositeMaterialGroupFactory;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterialGroupFactory

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DFaceReductionOptions;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions ABI::Windows::Graphics::Printing3D::IPrinting3DFaceReductionOptions

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial ABI::Windows::Graphics::Printing3D::IPrinting3DMaterial

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DMesh;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh ABI::Windows::Graphics::Printing3D::IPrinting3DMesh

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DMeshVerificationResult;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult ABI::Windows::Graphics::Printing3D::IPrinting3DMeshVerificationResult

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DModel;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel ABI::Windows::Graphics::Printing3D::IPrinting3DModel

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DModel2;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2 ABI::Windows::Graphics::Printing3D::IPrinting3DModel2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DModelTexture;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DMultiplePropertyMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterial

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DMultiplePropertyMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterialGroup

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DMultiplePropertyMaterialGroupFactory;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterialGroupFactory

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DTexture2CoordMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterial

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DTexture2CoordMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterialGroup

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DTexture2CoordMaterialGroup2;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2 ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterialGroup2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DTexture2CoordMaterialGroupFactory;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterialGroupFactory

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                interface IPrinting3DTextureResource;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_FWD_DEFINED__

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
        namespace Graphics {
            namespace Printing3D {
                class Printing3D3MFPackage;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6cf2eb38-e068-5558-94b0-0161192c5f19"))
IAsyncOperation<ABI::Windows::Graphics::Printing3D::Printing3D3MFPackage*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3D3MFPackage*, ABI::Windows::Graphics::Printing3D::IPrinting3D3MFPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Printing3D.Printing3D3MFPackage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Printing3D::Printing3D3MFPackage*> __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("28b6b208-85a7-53f1-83ae-577a7de66a9b"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Printing3D::Printing3D3MFPackage*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3D3MFPackage*, ABI::Windows::Graphics::Printing3D::IPrinting3D3MFPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Printing3D.Printing3D3MFPackage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Printing3D::Printing3D3MFPackage*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DMeshVerificationResult;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0f9eb6c4-19f5-5be9-9adb-64f24af115d8"))
IAsyncOperation<ABI::Windows::Graphics::Printing3D::Printing3DMeshVerificationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMeshVerificationResult*, ABI::Windows::Graphics::Printing3D::IPrinting3DMeshVerificationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Printing3D.Printing3DMeshVerificationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Printing3D::Printing3DMeshVerificationResult*> __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("186bae17-5896-56de-bff4-4f176b3e6194"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Printing3D::Printing3DMeshVerificationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMeshVerificationResult*, ABI::Windows::Graphics::Printing3D::IPrinting3DMeshVerificationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Printing3D.Printing3DMeshVerificationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Printing3D::Printing3DMeshVerificationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DModel;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_USE
#define DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1b27900b-10d5-53ff-9a34-4b31f31582b0"))
IAsyncOperation<ABI::Windows::Graphics::Printing3D::Printing3DModel*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DModel*, ABI::Windows::Graphics::Printing3D::IPrinting3DModel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Graphics.Printing3D.Printing3DModel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Graphics::Printing3D::Printing3DModel*> __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_t;
#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("26f4d34c-a11d-5b09-9908-ade8b1b13555"))
IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Printing3D::Printing3DModel*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DModel*, ABI::Windows::Graphics::Printing3D::IPrinting3DModel*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Graphics.Printing3D.Printing3DModel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Graphics::Printing3D::Printing3DModel*> __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("430ecece-1418-5d19-81b2-5ddb381603cc"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("398c4183-793d-5b00-819b-4aef92485e94"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0ec5345b-b37a-5cd5-83d7-9590cdf445b5"))
IAsyncOperationWithProgressCompletedHandler<bool, double> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Boolean, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<bool, double> __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_USE */



#ifndef DEF___FIAsyncOperationWithProgress_2_boolean_double_USE
#define DEF___FIAsyncOperationWithProgress_2_boolean_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("af873c66-2df0-5a95-ab54-25634da3ffa9"))
IAsyncOperationWithProgress<bool, double> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Boolean, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<bool, double> __FIAsyncOperationWithProgress_2_boolean_double_t;
#define __FIAsyncOperationWithProgress_2_boolean_double ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_boolean_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_boolean_double_USE */



#ifndef DEF___FIAsyncOperationProgressHandler_2_boolean_double_USE
#define DEF___FIAsyncOperationProgressHandler_2_boolean_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cadf3784-1200-5633-8280-163849914ab3"))
IAsyncOperationProgressHandler<bool, double> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Boolean, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<bool, double> __FIAsyncOperationProgressHandler_2_boolean_double_t;
#define __FIAsyncOperationProgressHandler_2_boolean_double ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_boolean_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_boolean_double_USE */



#ifndef DEF___FIIterator_1_double_USE
#define DEF___FIIterator_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("638a2cf4-f474-5318-9055-141cb909ac4b"))
IIterator<double> : IIterator_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<double> __FIIterator_1_double_t;
#define __FIIterator_1_double ABI::Windows::Foundation::Collections::__FIIterator_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_double_USE */



#ifndef DEF___FIIterable_1_double_USE
#define DEF___FIIterable_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c738964e-9c64-5bce-b5ce-61e9a282ec4a"))
IIterable<double> : IIterable_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<double> __FIIterable_1_double_t;
#define __FIIterable_1_double ABI::Windows::Foundation::Collections::__FIIterable_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_double_USE */



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



#ifndef DEF___FIIterator_1_UINT32_USE
#define DEF___FIIterator_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f06a2739-9443-5ef0-b284-dc5aff3e7d10"))
IIterator<UINT32> : IIterator_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<UINT32> __FIIterator_1_UINT32_t;
#define __FIIterator_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterator_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_UINT32_USE */



#ifndef DEF___FIIterable_1_UINT32_USE
#define DEF___FIIterable_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("421d4b91-b13b-5f37-ae54-b5249bd80539"))
IIterable<UINT32> : IIterable_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<UINT32> __FIIterable_1_UINT32_t;
#define __FIIterable_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterable_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_UINT32_USE */



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
        namespace Graphics {
            namespace Printing3D {
                class Printing3DBaseMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dad4dd0d-59ab-501f-9d6b-a209c7d54649"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DBaseMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9a6bd130-6f22-559c-b92c-14f9f8ddda47"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DBaseMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DBaseMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a34dc709-e2a7-5254-9dc1-cd47e85e2504"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c08f8e70-f6ef-5469-806a-7cb601dddb67"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DColorMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5a54a4a1-4d97-58d3-bdcc-1bf38b438d6d"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DColorMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c77d4f28-7882-52b4-b3c9-7d58c8836573"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DColorMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DColorMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("498467be-de0e-552b-b24e-8ee25ec9a486"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DColorMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1bf32a86-26ab-5750-b54c-3bda67867f8a"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DColorMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DComponent;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2e9eabba-184b-5c14-ae5f-eb634aa717e0"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DComponent*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DComponent*, ABI::Windows::Graphics::Printing3D::IPrinting3DComponent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DComponent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DComponent*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("516556ca-f862-59f8-8241-e0f0c177dadd"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DComponent*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DComponent*, ABI::Windows::Graphics::Printing3D::IPrinting3DComponent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DComponent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DComponent*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DComponentWithMatrix;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("01d2ce44-8b63-571f-b92e-bf2cf7cc6d53"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*, ABI::Windows::Graphics::Printing3D::IPrinting3DComponentWithMatrix*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DComponentWithMatrix>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8a213648-0b81-5e23-a48e-afe9f6691cc1"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*, ABI::Windows::Graphics::Printing3D::IPrinting3DComponentWithMatrix*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DComponentWithMatrix>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DCompositeMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b7e6b17a-a885-5c97-b29e-bf261eb5dad4"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DCompositeMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a0af2623-1b11-53cf-975d-64959386cdd3"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DCompositeMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DCompositeMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af86eea4-dd9d-5aa9-aee5-be3892124742"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f2ffef61-c254-58c0-8206-b3b3096be9cb"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DMesh;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("16ddf132-f80d-53b2-b09f-a42ed9689fc4"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DMesh*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMesh*, ABI::Windows::Graphics::Printing3D::IPrinting3DMesh*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DMesh>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DMesh*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a8018fda-de4d-56fa-8609-fd2298bfb558"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DMesh*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMesh*, ABI::Windows::Graphics::Printing3D::IPrinting3DMesh*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DMesh>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DMesh*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DModelTexture;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3d473ca2-4a8c-5cbd-807f-49af1580d2ba"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*, ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DModelTexture>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("94790870-6041-5d04-8699-17417117bb85"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*, ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DModelTexture>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DMultiplePropertyMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("614c0a0a-bf75-56ad-a304-b79f60017b83"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0dfc274e-ae4d-5bbb-93a8-7dc9f84ddac3"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DMultiplePropertyMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a24bab9a-d946-5036-b1c9-1c09b793f36c"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("23f7518e-2439-5573-a683-efca0c61a8d6"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DTexture2CoordMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("72d80d63-3626-5a2f-a579-78e70aa86d46"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("28373276-483c-5bd0-99c7-01bfa04a57d4"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DTexture2CoordMaterialGroup;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("11eaecc4-6ac1-5697-9bf5-1ef617e1dfeb"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("00017a1d-96bc-5c0e-b786-594fb4d077b6"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DTextureResource;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0678d5db-8fca-5084-a851-7312fe53f735"))
IIterator<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*, ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing3D.Printing3DTextureResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*> __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("54e3a71d-eae0-5199-9728-fac964850ebb"))
IIterable<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*, ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing3D.Printing3DTextureResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*> __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIVectorView_1_double_USE
#define DEF___FIVectorView_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af7586a8-6b21-5f61-bff1-1b682293ad96"))
IVectorView<double> : IVectorView_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<double> __FIVectorView_1_double_t;
#define __FIVectorView_1_double ABI::Windows::Foundation::Collections::__FIVectorView_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_double_USE */



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



#ifndef DEF___FIVectorView_1_UINT32_USE
#define DEF___FIVectorView_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e5ce1a07-8d33-5007-ba64-7d2508ccf85c"))
IVectorView<UINT32> : IVectorView_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<UINT32> __FIVectorView_1_UINT32_t;
#define __FIVectorView_1_UINT32 ABI::Windows::Foundation::Collections::__FIVectorView_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_UINT32_USE */


#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ccc6f0a2-4dd9-550d-8578-330e138ada07"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DBaseMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5c686c2e-cd88-5255-a961-5b4f2bf13c70"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("80da978e-7cc2-531f-816b-fa68aa446e8c"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DColorMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dcb0b640-8a0f-57c3-9e3e-d5a9881cc211"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DColorMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c3cc9d7c-b25f-5e52-9474-9d2915f44eac"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DComponent*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DComponent*, ABI::Windows::Graphics::Printing3D::IPrinting3DComponent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DComponent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DComponent*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0ed56a87-7746-5448-8d9d-dff9fa9fd760"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*, ABI::Windows::Graphics::Printing3D::IPrinting3DComponentWithMatrix*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DComponentWithMatrix>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dc940322-2b4f-5c6a-b7f6-358ced33bc68"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DCompositeMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("06715e0b-52f1-5d86-9eec-1f0797363383"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b836ce6-1d16-5e55-b1a0-61c70f29931b"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DMesh*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMesh*, ABI::Windows::Graphics::Printing3D::IPrinting3DMesh*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DMesh>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DMesh*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("338e034d-8832-5c05-81bf-27ab7f49264a"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*, ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DModelTexture>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac971e5e-ac32-587b-a701-fbbe6af1b112"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8513d339-12e9-5d0b-978d-fe967d051315"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("155500dd-f08a-5b43-8714-8bc01794ce23"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4d44ec40-4f39-5532-b9da-6a6f9a61a842"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0a4964cd-d387-5d71-a71d-4f0a55b9b689"))
IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*, ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Printing3D.Printing3DTextureResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*> __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_t;
#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVector_1_double_USE
#define DEF___FIVector_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f452d23c-bf05-5f3e-88e7-d17a6716b911"))
IVector<double> : IVector_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<double> __FIVector_1_double_t;
#define __FIVector_1_double ABI::Windows::Foundation::Collections::__FIVector_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_double_USE */



#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */



#ifndef DEF___FIVector_1_UINT32_USE
#define DEF___FIVector_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("534832ed-2a03-5604-890d-5a928cd427b9"))
IVector<UINT32> : IVector_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<UINT32> __FIVector_1_UINT32_t;
#define __FIVector_1_UINT32 ABI::Windows::Foundation::Collections::__FIVector_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_UINT32_USE */


#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6a5aa59f-fe10-517b-b1a9-c685ecce1644"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DBaseMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterial*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2b80d2cf-5449-5c81-8226-ebfc7d72f579"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DBaseMaterialGroup*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("606166fd-6bf5-53a1-b1ae-c34892ef1663"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DColorMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterial*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7c8017f3-8365-5aa8-9fd0-a769f26e3fef"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DColorMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DColorMaterialGroup*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("49e654c2-f372-582e-97cc-cb6b0fa3ba62"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DComponent*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DComponent*, ABI::Windows::Graphics::Printing3D::IPrinting3DComponent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DComponent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DComponent*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f09f3dd7-61e6-5a8d-9ddf-57001f705de7"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*, ABI::Windows::Graphics::Printing3D::IPrinting3DComponentWithMatrix*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DComponentWithMatrix>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DComponentWithMatrix*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c3b27a95-5efc-52c7-b5de-e82e059a722e"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DCompositeMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterial*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1e4ccd78-b6c0-51b1-ab2b-c3422f02c24e"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DCompositeMaterialGroup*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb11be6e-b592-5bc2-9a53-0127a9b32172"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DMesh*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMesh*, ABI::Windows::Graphics::Printing3D::IPrinting3DMesh*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DMesh>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DMesh*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e72578f-9bea-5663-8699-e7fcad3547a7"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*, ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DModelTexture>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DModelTexture*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2196da6-6a29-59a2-9dd6-93062f44baad"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterial*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("22585b94-34a1-5b6a-bae3-bf44659812f3"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DMultiplePropertyMaterialGroup*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f16fbf2c-c783-5edf-ad7b-7fb7eacf1501"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*, ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterial*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterial*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7dc68e96-2a62-5e7a-85d5-4864d03591eb"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*, ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterialGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DTexture2CoordMaterialGroup*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE
#define DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("120948c9-aaa5-5ee5-a133-3215d0561404"))
IVector<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*, ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Graphics.Printing3D.Printing3DTextureResource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Graphics::Printing3D::Printing3DTextureResource*> __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_t;
#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Print3DManager;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Print3DTaskRequestedEventArgs;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("77c464a3-a7c6-5512-9859-412db3f66ac4"))
ITypedEventHandler<ABI::Windows::Graphics::Printing3D::Print3DManager*, ABI::Windows::Graphics::Printing3D::Print3DTaskRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Print3DManager*, ABI::Windows::Graphics::Printing3D::IPrint3DManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Print3DTaskRequestedEventArgs*, ABI::Windows::Graphics::Printing3D::IPrint3DTaskRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing3D.Print3DManager, Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing3D::Print3DManager*, ABI::Windows::Graphics::Printing3D::Print3DTaskRequestedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Print3DTask;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c0081611-7485-58a8-88be-82e712d8c1ba"))
ITypedEventHandler<ABI::Windows::Graphics::Printing3D::Print3DTask*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Print3DTask*, ABI::Windows::Graphics::Printing3D::IPrint3DTask*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing3D.Print3DTask, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing3D::Print3DTask*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Print3DTaskCompletedEventArgs;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bccf7095-bc8e-5ff5-83c0-d5691e0aa24d"))
ITypedEventHandler<ABI::Windows::Graphics::Printing3D::Print3DTask*, ABI::Windows::Graphics::Printing3D::Print3DTaskCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Print3DTask*, ABI::Windows::Graphics::Printing3D::IPrint3DTask*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Print3DTaskCompletedEventArgs*, ABI::Windows::Graphics::Printing3D::IPrint3DTaskCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing3D.Print3DTask, Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing3D::Print3DTask*, ABI::Windows::Graphics::Printing3D::Print3DTaskCompletedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Print3DTaskSourceChangedEventArgs;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("58d36055-0241-555d-af7b-9f05e5daa412"))
ITypedEventHandler<ABI::Windows::Graphics::Printing3D::Print3DTask*, ABI::Windows::Graphics::Printing3D::Print3DTaskSourceChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Print3DTask*, ABI::Windows::Graphics::Printing3D::IPrint3DTask*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing3D::Print3DTaskSourceChangedEventArgs*, ABI::Windows::Graphics::Printing3D::IPrint3DTaskSourceChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing3D.Print3DTask, Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing3D::Print3DTask*, ABI::Windows::Graphics::Printing3D::Print3DTaskSourceChangedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_USE */

#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

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
            namespace Numerics {
                typedef struct Matrix4x4 Matrix4x4;
            } /* Numerics */
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
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef enum Print3DTaskCompletion : int Print3DTaskCompletion;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef enum Print3DTaskDetail : int Print3DTaskDetail;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef enum Printing3DBufferFormat : int Printing3DBufferFormat;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef enum Printing3DMeshVerificationMode : int Printing3DMeshVerificationMode;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef enum Printing3DModelUnit : int Printing3DModelUnit;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef enum Printing3DObjectType : int Printing3DObjectType;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef enum Printing3DPackageCompression : int Printing3DPackageCompression;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef enum Printing3DTextureEdgeBehavior : int Printing3DTextureEdgeBehavior;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                typedef struct Printing3DBufferDescription Printing3DBufferDescription;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Print3DTaskRequest;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Print3DTaskSourceRequestedArgs;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DFaceReductionOptions;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                class Printing3DMaterial;
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Printing3D.Print3DTaskCompletion
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                enum Print3DTaskCompletion : int
                {
                    Print3DTaskCompletion_Abandoned = 0,
                    Print3DTaskCompletion_Canceled = 1,
                    Print3DTaskCompletion_Failed = 2,
                    Print3DTaskCompletion_Slicing = 3,
                    Print3DTaskCompletion_Submitted = 4,
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Print3DTaskDetail
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                enum Print3DTaskDetail : int
                {
                    Print3DTaskDetail_Unknown = 0,
                    Print3DTaskDetail_ModelExceedsPrintBed = 1,
                    Print3DTaskDetail_UploadFailed = 2,
                    Print3DTaskDetail_InvalidMaterialSelection = 3,
                    Print3DTaskDetail_InvalidModel = 4,
                    Print3DTaskDetail_ModelNotManifold = 5,
                    Print3DTaskDetail_InvalidPrintTicket = 6,
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DBufferFormat
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                enum Printing3DBufferFormat : int
                {
                    Printing3DBufferFormat_Unknown = 0,
                    Printing3DBufferFormat_R32G32B32A32Float = 2,
                    Printing3DBufferFormat_R32G32B32A32UInt = 3,
                    Printing3DBufferFormat_R32G32B32Float = 6,
                    Printing3DBufferFormat_R32G32B32UInt = 7,
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
                    Printing3DBufferFormat_Printing3DDouble = 500,
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
                    Printing3DBufferFormat_Printing3DUInt = 501,
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DMeshVerificationMode
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                enum Printing3DMeshVerificationMode : int
                {
                    Printing3DMeshVerificationMode_FindFirstError = 0,
                    Printing3DMeshVerificationMode_FindAllErrors = 1,
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DModelUnit
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                enum Printing3DModelUnit : int
                {
                    Printing3DModelUnit_Meter = 0,
                    Printing3DModelUnit_Micron = 1,
                    Printing3DModelUnit_Millimeter = 2,
                    Printing3DModelUnit_Centimeter = 3,
                    Printing3DModelUnit_Inch = 4,
                    Printing3DModelUnit_Foot = 5,
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DObjectType
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                enum Printing3DObjectType : int
                {
                    Printing3DObjectType_Model = 0,
                    Printing3DObjectType_Support = 1,
                    Printing3DObjectType_Others = 2,
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DPackageCompression
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 4.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                enum Printing3DPackageCompression : int
                {
                    Printing3DPackageCompression_Low = 0,
                    Printing3DPackageCompression_Medium = 1,
                    Printing3DPackageCompression_High = 2,
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DTextureEdgeBehavior
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                enum Printing3DTextureEdgeBehavior : int
                {
                    Printing3DTextureEdgeBehavior_None = 0,
                    Printing3DTextureEdgeBehavior_Wrap = 1,
                    Printing3DTextureEdgeBehavior_Mirror = 2,
                    Printing3DTextureEdgeBehavior_Clamp = 3,
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DBufferDescription
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                struct Printing3DBufferDescription
                {
                    ABI::Windows::Graphics::Printing3D::Printing3DBufferFormat Format;
                    UINT32 Stride;
                };
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Graphics.Printing3D.Print3DTaskSourceRequestedHandler
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("e9175e70-c917-46de-bb51-d9a94db3711f")
                IPrint3DTaskSourceRequestedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Graphics::Printing3D::IPrint3DTaskSourceRequestedArgs* args
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DTaskSourceRequestedHandler = __uuidof(IPrint3DTaskSourceRequestedHandler);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DManager
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DManager
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DManager[] = L"Windows.Graphics.Printing3D.IPrint3DManager";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("4d2fcb0a-7366-4971-8bd5-17c4e3e8c6c0")
                IPrint3DManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_TaskRequested(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs* eventHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TaskRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DManager = __uuidof(IPrint3DManager);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DManagerStatics
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DManager
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DManagerStatics[] = L"Windows.Graphics.Printing3D.IPrint3DManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("0ef1cafe-a9ad-4c08-a917-1d1f863eabcb")
                IPrint3DManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::Graphics::Printing3D::IPrint3DManager** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowPrintUIAsync(
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DManagerStatics = __uuidof(IPrint3DManagerStatics);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTask
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTask
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTask[] = L"Windows.Graphics.Printing3D.IPrint3DTask";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("8ce3d080-2118-4c28-80de-f426d70191ae")
                IPrint3DTask : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::Graphics::Printing3D::IPrinting3D3MFPackage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Submitting(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Submitting(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Completed(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Completed(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SourceChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SourceChanged(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DTask = __uuidof(IPrint3DTask);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskCompletedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskCompletedEventArgs[] = L"Windows.Graphics.Printing3D.IPrint3DTaskCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("cc1914af-2614-4f1d-accc-d6fc4fda5455")
                IPrint3DTaskCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Completion(
                        ABI::Windows::Graphics::Printing3D::Print3DTaskCompletion* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedStatus(
                        ABI::Windows::Graphics::Printing3D::Print3DTaskDetail* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DTaskCompletedEventArgs = __uuidof(IPrint3DTaskCompletedEventArgs);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskRequest
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskRequest
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskRequest[] = L"Windows.Graphics.Printing3D.IPrint3DTaskRequest";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("2595c46f-2245-4c5a-8731-0d604dc6bc3c")
                IPrint3DTaskRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateTask(
                        HSTRING title,
                        HSTRING printerId,
                        ABI::Windows::Graphics::Printing3D::IPrint3DTaskSourceRequestedHandler* handler,
                        ABI::Windows::Graphics::Printing3D::IPrint3DTask** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DTaskRequest = __uuidof(IPrint3DTaskRequest);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskRequestedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskRequestedEventArgs[] = L"Windows.Graphics.Printing3D.IPrint3DTaskRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("150cb77f-18c5-40d7-9f40-fab3096e05a9")
                IPrint3DTaskRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        ABI::Windows::Graphics::Printing3D::IPrint3DTaskRequest** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DTaskRequestedEventArgs = __uuidof(IPrint3DTaskRequestedEventArgs);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskSourceChangedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskSourceChangedEventArgs[] = L"Windows.Graphics.Printing3D.IPrint3DTaskSourceChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("5bcd34af-24e9-4c10-8d07-14c346ba3fcf")
                IPrint3DTaskSourceChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::Graphics::Printing3D::IPrinting3D3MFPackage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DTaskSourceChangedEventArgs = __uuidof(IPrint3DTaskSourceChangedEventArgs);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskSourceRequestedArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskSourceRequestedArgs[] = L"Windows.Graphics.Printing3D.IPrint3DTaskSourceRequestedArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("c77c9aba-24af-424d-a3bf-92250c355602")
                IPrint3DTaskSourceRequestedArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetSource(
                        ABI::Windows::Graphics::Printing3D::IPrinting3D3MFPackage* source
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DTaskSourceRequestedArgs = __uuidof(IPrint3DTaskSourceRequestedArgs);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3D3MFPackage
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3D3MFPackage
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3D3MFPackage[] = L"Windows.Graphics.Printing3D.IPrinting3D3MFPackage";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("f64dd5c8-2ab7-45a9-a1b7-267e948d5b18")
                IPrinting3D3MFPackage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrintTicket(
                        ABI::Windows::Storage::Streams::IRandomAccessStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PrintTicket(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModelPart(
                        ABI::Windows::Storage::Streams::IRandomAccessStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ModelPart(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Thumbnail(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Textures(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadModelFromPackageAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* value,
                        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveModelToPackageAsync(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DModel* value,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3D3MFPackage = __uuidof(IPrinting3D3MFPackage);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3D3MFPackage2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3D3MFPackage
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3D3MFPackage2[] = L"Windows.Graphics.Printing3D.IPrinting3D3MFPackage2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("965c7ac4-93cb-4430-92b8-789cd454f883")
                IPrinting3D3MFPackage2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Compression(
                        ABI::Windows::Graphics::Printing3D::Printing3DPackageCompression* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Compression(
                        ABI::Windows::Graphics::Printing3D::Printing3DPackageCompression value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3D3MFPackage2 = __uuidof(IPrinting3D3MFPackage2);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3D3MFPackageStatics
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3D3MFPackage
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3D3MFPackageStatics[] = L"Windows.Graphics.Printing3D.IPrinting3D3MFPackageStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("7058d9af-7a9a-4787-b817-f6f459214823")
                IPrinting3D3MFPackageStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE LoadAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* value,
                        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3D3MFPackageStatics = __uuidof(IPrinting3D3MFPackageStatics);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DBaseMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DBaseMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DBaseMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DBaseMaterial";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("d0f0e743-c50c-4bcb-9d04-fc16adcea2c9")
                IPrinting3DBaseMaterial : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Color(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterial** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Color(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterial* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DBaseMaterial = __uuidof(IPrinting3DBaseMaterial);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DBaseMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroup";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("94f070b8-2515-4a8d-a1f0-d0fc13d06021")
                IPrinting3DBaseMaterialGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Bases(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaterialGroupId(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DBaseMaterialGroup = __uuidof(IPrinting3DBaseMaterialGroup);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DBaseMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("5c1546dc-8697-4193-976b-84bb4116e5bf")
                IPrinting3DBaseMaterialGroupFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        UINT32 MaterialGroupId,
                        ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroup** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DBaseMaterialGroupFactory = __uuidof(IPrinting3DBaseMaterialGroupFactory);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DBaseMaterialStatics
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DBaseMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DBaseMaterialStatics[] = L"Windows.Graphics.Printing3D.IPrinting3DBaseMaterialStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("815a47bc-374a-476d-be92-3ecfd1cb9776")
                IPrinting3DBaseMaterialStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Abs(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pla(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DBaseMaterialStatics = __uuidof(IPrinting3DBaseMaterialStatics);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DColorMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DColorMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DColorMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DColorMaterial";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("e1899928-7ce7-4285-a35d-f145c9510c7b")
                IPrinting3DColorMaterial : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DColorMaterial = __uuidof(IPrinting3DColorMaterial);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DColorMaterial2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DColorMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DColorMaterial2[] = L"Windows.Graphics.Printing3D.IPrinting3DColorMaterial2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("fab0e852-0aef-44e9-9ddd-36eeea5acd44")
                IPrinting3DColorMaterial2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Color(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Color(
                        ABI::Windows::UI::Color value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DColorMaterial2 = __uuidof(IPrinting3DColorMaterial2);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DColorMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DColorMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroup";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("001a6bd0-aadf-4226-afe9-f369a0b45004")
                IPrinting3DColorMaterialGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Colors(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaterialGroupId(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DColorMaterialGroup = __uuidof(IPrinting3DColorMaterialGroup);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DColorMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DColorMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("71d38d6d-b1ea-4a5b-bc54-19c65f3df044")
                IPrinting3DColorMaterialGroupFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        UINT32 MaterialGroupId,
                        ABI::Windows::Graphics::Printing3D::IPrinting3DColorMaterialGroup** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DColorMaterialGroupFactory = __uuidof(IPrinting3DColorMaterialGroupFactory);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DComponent
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DComponent
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DComponent[] = L"Windows.Graphics.Printing3D.IPrinting3DComponent";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("7e287845-bf7f-4cdb-a27f-30a01437fede")
                IPrinting3DComponent : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Mesh(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DMesh** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mesh(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DMesh* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Components(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Thumbnail(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Graphics::Printing3D::Printing3DObjectType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Type(
                        ABI::Windows::Graphics::Printing3D::Printing3DObjectType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PartNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PartNumber(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DComponent = __uuidof(IPrinting3DComponent);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DComponentWithMatrix
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DComponentWithMatrix
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DComponentWithMatrix[] = L"Windows.Graphics.Printing3D.IPrinting3DComponentWithMatrix";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("3279f335-0ef0-456b-9a21-49bebe8b51c2")
                IPrinting3DComponentWithMatrix : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Component(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DComponent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Component(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DComponent* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Matrix(
                        ABI::Windows::Foundation::Numerics::Matrix4x4* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Matrix(
                        ABI::Windows::Foundation::Numerics::Matrix4x4 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DComponentWithMatrix = __uuidof(IPrinting3DComponentWithMatrix);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DCompositeMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DCompositeMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DCompositeMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DCompositeMaterial";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("462238dd-562e-4f6c-882d-f4d841fd63c7")
                IPrinting3DCompositeMaterial : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Values(
                        __FIVector_1_double** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DCompositeMaterial = __uuidof(IPrinting3DCompositeMaterial);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DCompositeMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("8d946a5b-40f1-496d-a5fb-340a5a678e30")
                IPrinting3DCompositeMaterialGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Composites(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaterialGroupId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaterialIndices(
                        __FIVector_1_UINT32** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DCompositeMaterialGroup = __uuidof(IPrinting3DCompositeMaterialGroup);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DCompositeMaterialGroup2[] = L"Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("06e86d62-7d3b-41e1-944c-bafde4555483")
                IPrinting3DCompositeMaterialGroup2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BaseMaterialGroup(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroup** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BaseMaterialGroup(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DBaseMaterialGroup* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DCompositeMaterialGroup2 = __uuidof(IPrinting3DCompositeMaterialGroup2);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DCompositeMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("d08ecd13-92ff-43aa-a627-8d43c22c817e")
                IPrinting3DCompositeMaterialGroupFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        UINT32 MaterialGroupId,
                        ABI::Windows::Graphics::Printing3D::IPrinting3DCompositeMaterialGroup** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DCompositeMaterialGroupFactory = __uuidof(IPrinting3DCompositeMaterialGroupFactory);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DFaceReductionOptions
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DFaceReductionOptions
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DFaceReductionOptions[] = L"Windows.Graphics.Printing3D.IPrinting3DFaceReductionOptions";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("bbfed397-2d74-46f7-be85-99a67bbb6629")
                IPrinting3DFaceReductionOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxReductionArea(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxReductionArea(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TargetTriangleCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TargetTriangleCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxEdgeLength(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxEdgeLength(
                        DOUBLE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DFaceReductionOptions = __uuidof(IPrinting3DFaceReductionOptions);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DMaterial";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("378db256-ed62-4952-b85b-03567d7c465e")
                IPrinting3DMaterial : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BaseGroups(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ColorGroups(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Texture2CoordGroups(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CompositeGroups(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MultiplePropertyGroups(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DMaterial = __uuidof(IPrinting3DMaterial);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMesh
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMesh
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMesh[] = L"Windows.Graphics.Printing3D.IPrinting3DMesh";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("192e90dc-0228-2e01-bc20-c5290cbf32c4")
                IPrinting3DMesh : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VertexCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_VertexCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IndexCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IndexCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VertexPositionsDescription(
                        ABI::Windows::Graphics::Printing3D::Printing3DBufferDescription* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_VertexPositionsDescription(
                        ABI::Windows::Graphics::Printing3D::Printing3DBufferDescription value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VertexNormalsDescription(
                        ABI::Windows::Graphics::Printing3D::Printing3DBufferDescription* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_VertexNormalsDescription(
                        ABI::Windows::Graphics::Printing3D::Printing3DBufferDescription value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TriangleIndicesDescription(
                        ABI::Windows::Graphics::Printing3D::Printing3DBufferDescription* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TriangleIndicesDescription(
                        ABI::Windows::Graphics::Printing3D::Printing3DBufferDescription value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TriangleMaterialIndicesDescription(
                        ABI::Windows::Graphics::Printing3D::Printing3DBufferDescription* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TriangleMaterialIndicesDescription(
                        ABI::Windows::Graphics::Printing3D::Printing3DBufferDescription value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVertexPositions(
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateVertexPositions(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVertexNormals(
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateVertexNormals(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTriangleIndices(
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTriangleIndices(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTriangleMaterialIndices(
                        ABI::Windows::Storage::Streams::IBuffer** buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTriangleMaterialIndices(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BufferDescriptionSet(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BufferSet(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE VerifyAsync(
                        ABI::Windows::Graphics::Printing3D::Printing3DMeshVerificationMode value,
                        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DMesh = __uuidof(IPrinting3DMesh);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMeshVerificationResult
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMeshVerificationResult
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMeshVerificationResult[] = L"Windows.Graphics.Printing3D.IPrinting3DMeshVerificationResult";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("195671ba-e93a-4e8a-a46f-dea8e852197e")
                IPrinting3DMeshVerificationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsValid(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NonmanifoldTriangles(
                        __FIVectorView_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReversedNormalTriangles(
                        __FIVectorView_1_UINT32** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DMeshVerificationResult = __uuidof(IPrinting3DMeshVerificationResult);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DModel
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DModel
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DModel[] = L"Windows.Graphics.Printing3D.IPrinting3DModel";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("2d012ef0-52fb-919a-77b0-4b1a3b80324f")
                IPrinting3DModel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Unit(
                        ABI::Windows::Graphics::Printing3D::Printing3DModelUnit* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Unit(
                        ABI::Windows::Graphics::Printing3D::Printing3DModelUnit value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Textures(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Meshes(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Components(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Material(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DMaterial** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Material(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DMaterial* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Build(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DComponent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Build(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DComponent* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Version(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Version(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequiredExtensions(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Metadata(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RepairAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clone(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DModel** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DModel = __uuidof(IPrinting3DModel);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DModel2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DModel
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DModel2[] = L"Windows.Graphics.Printing3D.IPrinting3DModel2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("c92069c7-c841-47f3-a84e-a149fd08b657")
                IPrinting3DModel2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryPartialRepairAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryPartialRepairWithTimeAsync(
                        ABI::Windows::Foundation::TimeSpan maxWaitTime,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryReduceFacesAsync(
                        __FIAsyncOperationWithProgress_2_boolean_double** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryReduceFacesWithOptionsAsync(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DFaceReductionOptions* printing3DFaceReductionOptions,
                        __FIAsyncOperationWithProgress_2_boolean_double** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryReduceFacesWithOptionsAndTimeAsync(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DFaceReductionOptions* printing3DFaceReductionOptions,
                        ABI::Windows::Foundation::TimeSpan maxWait,
                        __FIAsyncOperationWithProgress_2_boolean_double** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RepairWithProgressAsync(
                        __FIAsyncOperationWithProgress_2_boolean_double** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DModel2 = __uuidof(IPrinting3DModel2);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DModelTexture
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DModelTexture
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DModelTexture[] = L"Windows.Graphics.Printing3D.IPrinting3DModelTexture";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("5dafcf01-b59d-483c-97bb-a4d546d1c75c")
                IPrinting3DModelTexture : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TextureResource(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TextureResource(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DTextureResource* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TileStyleU(
                        ABI::Windows::Graphics::Printing3D::Printing3DTextureEdgeBehavior* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TileStyleU(
                        ABI::Windows::Graphics::Printing3D::Printing3DTextureEdgeBehavior value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TileStyleV(
                        ABI::Windows::Graphics::Printing3D::Printing3DTextureEdgeBehavior* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TileStyleV(
                        ABI::Windows::Graphics::Printing3D::Printing3DTextureEdgeBehavior value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DModelTexture = __uuidof(IPrinting3DModelTexture);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMultiplePropertyMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterial";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("25a6254b-c6e9-484d-a214-a25e5776ba62")
                IPrinting3DMultiplePropertyMaterial : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaterialIndices(
                        __FIVector_1_UINT32** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DMultiplePropertyMaterial = __uuidof(IPrinting3DMultiplePropertyMaterial);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMultiplePropertyMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroup";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("f0950519-aeb9-4515-a39b-a088fbbb277c")
                IPrinting3DMultiplePropertyMaterialGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MultipleProperties(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaterialGroupIndices(
                        __FIVector_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaterialGroupId(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DMultiplePropertyMaterialGroup = __uuidof(IPrinting3DMultiplePropertyMaterialGroup);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMultiplePropertyMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("323e196e-d4c6-451e-a814-4d78a210fe53")
                IPrinting3DMultiplePropertyMaterialGroupFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        UINT32 MaterialGroupId,
                        ABI::Windows::Graphics::Printing3D::IPrinting3DMultiplePropertyMaterialGroup** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DMultiplePropertyMaterialGroupFactory = __uuidof(IPrinting3DMultiplePropertyMaterialGroupFactory);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTexture2CoordMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterial";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("8d844bfb-07e9-4986-9833-8dd3d48c6859")
                IPrinting3DTexture2CoordMaterial : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Texture(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Texture(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_U(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_U(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_V(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_V(
                        DOUBLE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DTexture2CoordMaterial = __uuidof(IPrinting3DTexture2CoordMaterial);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTexture2CoordMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("627d7ca7-6d90-4fb9-9fc4-9feff3dfa892")
                IPrinting3DTexture2CoordMaterialGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Texture2Coords(
                        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaterialGroupId(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DTexture2CoordMaterialGroup = __uuidof(IPrinting3DTexture2CoordMaterialGroup);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTexture2CoordMaterialGroup2[] = L"Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("69fbdbba-b12e-429b-8386-df5284f6e80f")
                IPrinting3DTexture2CoordMaterialGroup2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Texture(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Texture(
                        ABI::Windows::Graphics::Printing3D::IPrinting3DModelTexture* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DTexture2CoordMaterialGroup2 = __uuidof(IPrinting3DTexture2CoordMaterialGroup2);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTexture2CoordMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("cbb049b0-468a-4c6f-b2a2-8eb8ba8dea48")
                IPrinting3DTexture2CoordMaterialGroupFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        UINT32 MaterialGroupId,
                        ABI::Windows::Graphics::Printing3D::IPrinting3DTexture2CoordMaterialGroup** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DTexture2CoordMaterialGroupFactory = __uuidof(IPrinting3DTexture2CoordMaterialGroupFactory);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTextureResource
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTextureResource
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTextureResource[] = L"Windows.Graphics.Printing3D.IPrinting3DTextureResource";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing3D {
                MIDL_INTERFACE("a70df32d-6ab1-44ae-bc45-a27382c0d38c")
                IPrinting3DTextureResource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TextureData(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TextureData(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrinting3DTextureResource = __uuidof(IPrinting3DTextureResource);
            } /* Printing3D */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DManager
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing3D.IPrint3DManagerStatics interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DManager_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DManager[] = L"Windows.Graphics.Printing3D.Print3DManager";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTask
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTask ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTask_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTask[] = L"Windows.Graphics.Printing3D.Print3DTask";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskCompletedEventArgs[] = L"Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskRequest
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskRequest[] = L"Windows.Graphics.Printing3D.Print3DTaskRequest";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskRequestedEventArgs[] = L"Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskSourceChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskSourceChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskSourceChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskSourceChangedEventArgs[] = L"Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskSourceRequestedArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskSourceRequestedArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskSourceRequestedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskSourceRequestedArgs[] = L"Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3D3MFPackage
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing3D.IPrinting3D3MFPackageStatics interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3D3MFPackage ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3D3MFPackage2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3D3MFPackage_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3D3MFPackage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3D3MFPackage[] = L"Windows.Graphics.Printing3D.Printing3D3MFPackage";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DBaseMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing3D.IPrinting3DBaseMaterialStatics interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DBaseMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DBaseMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DBaseMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DBaseMaterial[] = L"Windows.Graphics.Printing3D.Printing3DBaseMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DBaseMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DBaseMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DBaseMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DColorMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DColorMaterial ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3DColorMaterial2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DColorMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DColorMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DColorMaterial[] = L"Windows.Graphics.Printing3D.Printing3DColorMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DColorMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DColorMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DColorMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DColorMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DColorMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DComponent
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DComponent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DComponent_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DComponent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DComponent[] = L"Windows.Graphics.Printing3D.Printing3DComponent";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DComponentWithMatrix
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DComponentWithMatrix ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DComponentWithMatrix_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DComponentWithMatrix_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DComponentWithMatrix[] = L"Windows.Graphics.Printing3D.Printing3DComponentWithMatrix";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DCompositeMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DCompositeMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DCompositeMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DCompositeMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DCompositeMaterial[] = L"Windows.Graphics.Printing3D.Printing3DCompositeMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DCompositeMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DCompositeMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DCompositeMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DFaceReductionOptions
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DFaceReductionOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DFaceReductionOptions_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DFaceReductionOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DFaceReductionOptions[] = L"Windows.Graphics.Printing3D.Printing3DFaceReductionOptions";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMaterial[] = L"Windows.Graphics.Printing3D.Printing3DMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMesh
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMesh ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMesh_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMesh_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMesh[] = L"Windows.Graphics.Printing3D.Printing3DMesh";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMeshVerificationResult
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMeshVerificationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMeshVerificationResult_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMeshVerificationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMeshVerificationResult[] = L"Windows.Graphics.Printing3D.Printing3DMeshVerificationResult";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DModel
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DModel ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3DModel2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DModel_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DModel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DModel[] = L"Windows.Graphics.Printing3D.Printing3DModel";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DModelTexture
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DModelTexture ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DModelTexture_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DModelTexture_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DModelTexture[] = L"Windows.Graphics.Printing3D.Printing3DModelTexture";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterial[] = L"Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterial[] = L"Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DTextureResource
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DTextureResource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTextureResource_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTextureResource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DTextureResource[] = L"Windows.Graphics.Printing3D.Printing3DTextureResource";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2 __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2 __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2 __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2 __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2 __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage;

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackageVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* This,
        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackageVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult;

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResultVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* This,
        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel;

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel;

typedef struct __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelVtbl;

interface __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* This,
        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_boolean_double __FIAsyncOperationProgressHandler_2_boolean_double;

typedef interface __FIAsyncOperationWithProgress_2_boolean_double __FIAsyncOperationWithProgress_2_boolean_double;

#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_boolean_double;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_boolean_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_boolean_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_boolean_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_boolean_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_boolean_double* This,
        __FIAsyncOperationWithProgress_2_boolean_double* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_boolean_doubleVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_boolean_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_boolean_double_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationWithProgress_2_boolean_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_boolean_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_boolean_double __FIAsyncOperationWithProgress_2_boolean_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_boolean_double;

typedef struct __FIAsyncOperationWithProgress_2_boolean_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_boolean_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_boolean_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        __FIAsyncOperationProgressHandler_2_boolean_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        __FIAsyncOperationProgressHandler_2_boolean_double** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_boolean_double** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_boolean_double* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_boolean_doubleVtbl;

interface __FIAsyncOperationWithProgress_2_boolean_double
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_boolean_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_boolean_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_boolean_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_boolean_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_boolean_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_boolean_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_boolean_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_boolean_double_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_boolean_double_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_boolean_double_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_boolean_double_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_boolean_double_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_boolean_double_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationProgressHandler_2_boolean_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_boolean_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_boolean_double __FIAsyncOperationProgressHandler_2_boolean_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_boolean_double;

typedef struct __FIAsyncOperationProgressHandler_2_boolean_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_boolean_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_boolean_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_boolean_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_boolean_double* This,
        __FIAsyncOperationWithProgress_2_boolean_double* asyncInfo,
        DOUBLE progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_boolean_doubleVtbl;

interface __FIAsyncOperationProgressHandler_2_boolean_double
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_boolean_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_boolean_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_boolean_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_boolean_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_boolean_double_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_boolean_double_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_double_INTERFACE_DEFINED__)
#define ____FIIterator_1_double_INTERFACE_DEFINED__

typedef interface __FIIterator_1_double __FIIterator_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_double;

typedef struct __FIIterator_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_double* This,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_double* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_double* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_double* This,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_doubleVtbl;

interface __FIIterator_1_double
{
    CONST_VTBL struct __FIIterator_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_double_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_double_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_double_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_double_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_double_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_double_INTERFACE_DEFINED__)
#define ____FIIterable_1_double_INTERFACE_DEFINED__

typedef interface __FIIterable_1_double __FIIterable_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_double;

typedef struct __FIIterable_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_double* This,
        __FIIterator_1_double** result);

    END_INTERFACE
} __FIIterable_1_doubleVtbl;

interface __FIIterable_1_double
{
    CONST_VTBL struct __FIIterable_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_double_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_double_INTERFACE_DEFINED__

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

#if !defined(____FIIterator_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterator_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterator_1_UINT32 __FIIterator_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_UINT32;

typedef struct __FIIterator_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_UINT32* This,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_UINT32Vtbl;

interface __FIIterator_1_UINT32
{
    CONST_VTBL struct __FIIterator_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_UINT32_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_UINT32_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_UINT32_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_UINT32_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterable_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterable_1_UINT32 __FIIterable_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_UINT32;

typedef struct __FIIterable_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_UINT32* This,
        __FIIterator_1_UINT32** result);

    END_INTERFACE
} __FIIterable_1_UINT32Vtbl;

interface __FIIterable_1_UINT32
{
    CONST_VTBL struct __FIIterable_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_UINT32_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_UINT32_INTERFACE_DEFINED__

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

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        __FIIterator_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

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

#if !defined(____FIVectorView_1_double_INTERFACE_DEFINED__)
#define ____FIVectorView_1_double_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_double __FIVectorView_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_double;

typedef struct __FIVectorView_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_double* This,
        UINT32 index,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_double* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_double* This,
        DOUBLE value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_double* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_doubleVtbl;

interface __FIVectorView_1_double
{
    CONST_VTBL struct __FIVectorView_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_double_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_double_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_double_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_double_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_double_INTERFACE_DEFINED__

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

#if !defined(____FIVectorView_1_UINT32_INTERFACE_DEFINED__)
#define ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_UINT32 __FIVectorView_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_UINT32;

typedef struct __FIVectorView_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_UINT32* This,
        UINT32 index,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_UINT32* This,
        UINT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_UINT32* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_UINT32Vtbl;

interface __FIVectorView_1_UINT32
{
    CONST_VTBL struct __FIVectorView_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_UINT32_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_UINT32_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_UINT32_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_UINT32_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource;

typedef struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl;

interface __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if !defined(____FIVector_1_double_INTERFACE_DEFINED__)
#define ____FIVector_1_double_INTERFACE_DEFINED__

typedef interface __FIVector_1_double __FIVector_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_double;

typedef struct __FIVector_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_double* This,
        UINT32 index,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_double* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_double* This,
        __FIVectorView_1_double** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_double* This,
        DOUBLE value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_double* This,
        UINT32 index,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_double* This,
        UINT32 index,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_double* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_double* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_double* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_double* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_double* This,
        UINT32 itemsLength,
        DOUBLE* items);

    END_INTERFACE
} __FIVector_1_doubleVtbl;

interface __FIVector_1_double
{
    CONST_VTBL struct __FIVector_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_double_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_double_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_double_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_double_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_double_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_double_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_double_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_double_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_double_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_double_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_double_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_double_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_double_INTERFACE_DEFINED__

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIVector_1_UINT32_INTERFACE_DEFINED__)
#define ____FIVector_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIVector_1_UINT32 __FIVector_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_UINT32;

typedef struct __FIVector_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_UINT32* This,
        UINT32 index,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_UINT32* This,
        __FIVectorView_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_UINT32* This,
        UINT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_UINT32* This,
        UINT32 index,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_UINT32* This,
        UINT32 index,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_UINT32* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_UINT32* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_UINT32* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_UINT32* This,
        UINT32 itemsLength,
        UINT32* items);

    END_INTERFACE
} __FIVector_1_UINT32Vtbl;

interface __FIVector_1_UINT32
{
    CONST_VTBL struct __FIVector_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_UINT32_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_UINT32_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_UINT32_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_UINT32_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_UINT32_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_UINT32_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_UINT32_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_UINT32_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_UINT32_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_UINT32_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_UINT32_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_UINT32_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_UINT32_INTERFACE_DEFINED__

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrixVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTextureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource;

typedef struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        __FIVectorView_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** items);

    END_INTERFACE
} __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl;

interface __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource
{
    CONST_VTBL struct __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* sender,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* sender,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* sender,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4 __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrint3DTaskCompletion __x_ABI_CWindows_CGraphics_CPrinting3D_CPrint3DTaskCompletion;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrint3DTaskDetail __x_ABI_CWindows_CGraphics_CPrinting3D_CPrint3DTaskDetail;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferFormat __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferFormat;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DMeshVerificationMode __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DMeshVerificationMode;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DModelUnit __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DModelUnit;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DObjectType __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DObjectType;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DPackageCompression __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DPackageCompression;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DTextureEdgeBehavior __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DTextureEdgeBehavior;

typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription;

/*
 *
 * Struct Windows.Graphics.Printing3D.Print3DTaskCompletion
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrint3DTaskCompletion
{
    Print3DTaskCompletion_Abandoned = 0,
    Print3DTaskCompletion_Canceled = 1,
    Print3DTaskCompletion_Failed = 2,
    Print3DTaskCompletion_Slicing = 3,
    Print3DTaskCompletion_Submitted = 4,
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Print3DTaskDetail
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrint3DTaskDetail
{
    Print3DTaskDetail_Unknown = 0,
    Print3DTaskDetail_ModelExceedsPrintBed = 1,
    Print3DTaskDetail_UploadFailed = 2,
    Print3DTaskDetail_InvalidMaterialSelection = 3,
    Print3DTaskDetail_InvalidModel = 4,
    Print3DTaskDetail_ModelNotManifold = 5,
    Print3DTaskDetail_InvalidPrintTicket = 6,
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DBufferFormat
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferFormat
{
    Printing3DBufferFormat_Unknown = 0,
    Printing3DBufferFormat_R32G32B32A32Float = 2,
    Printing3DBufferFormat_R32G32B32A32UInt = 3,
    Printing3DBufferFormat_R32G32B32Float = 6,
    Printing3DBufferFormat_R32G32B32UInt = 7,
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
    Printing3DBufferFormat_Printing3DDouble = 500,
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
    Printing3DBufferFormat_Printing3DUInt = 501,
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DMeshVerificationMode
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DMeshVerificationMode
{
    Printing3DMeshVerificationMode_FindFirstError = 0,
    Printing3DMeshVerificationMode_FindAllErrors = 1,
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DModelUnit
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DModelUnit
{
    Printing3DModelUnit_Meter = 0,
    Printing3DModelUnit_Micron = 1,
    Printing3DModelUnit_Millimeter = 2,
    Printing3DModelUnit_Centimeter = 3,
    Printing3DModelUnit_Inch = 4,
    Printing3DModelUnit_Foot = 5,
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DObjectType
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DObjectType
{
    Printing3DObjectType_Model = 0,
    Printing3DObjectType_Support = 1,
    Printing3DObjectType_Others = 2,
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DPackageCompression
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 4.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DPackageCompression
{
    Printing3DPackageCompression_Low = 0,
    Printing3DPackageCompression_Medium = 1,
    Printing3DPackageCompression_High = 2,
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DTextureEdgeBehavior
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DTextureEdgeBehavior
{
    Printing3DTextureEdgeBehavior_None = 0,
    Printing3DTextureEdgeBehavior_Wrap = 1,
    Printing3DTextureEdgeBehavior_Mirror = 2,
    Printing3DTextureEdgeBehavior_Clamp = 3,
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing3D.Printing3DBufferDescription
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription
{
    enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferFormat Format;
    UINT32 Stride;
};
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Graphics.Printing3D.Print3DTaskSourceRequestedHandler
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs* args);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandlerVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_Invoke(This, args) \
    ((This)->lpVtbl->Invoke(This, args))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DManager
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DManager
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DManager[] = L"Windows.Graphics.Printing3D.IPrint3DManager";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_TaskRequested)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DManager_Windows__CGraphics__CPrinting3D__CPrint3DTaskRequestedEventArgs* eventHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TaskRequested)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_add_TaskRequested(This, eventHandler, token) \
    ((This)->lpVtbl->add_TaskRequested(This, eventHandler, token))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_remove_TaskRequested(This, token) \
    ((This)->lpVtbl->remove_TaskRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DManagerStatics
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DManager
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DManagerStatics[] = L"Windows.Graphics.Printing3D.IPrint3DManagerStatics";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManager** result);
    HRESULT (STDMETHODCALLTYPE* ShowPrintUIAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics* This,
        __FIAsyncOperation_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_ShowPrintUIAsync(This, result) \
    ((This)->lpVtbl->ShowPrintUIAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTask
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTask
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTask[] = L"Windows.Graphics.Printing3D.IPrint3DTask";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage** value);
    HRESULT (STDMETHODCALLTYPE* add_Submitting)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_IInspectable* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Submitting)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* add_Completed)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskCompletedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Completed)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* add_SourceChanged)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting3D__CPrint3DTask_Windows__CGraphics__CPrinting3D__CPrint3DTaskSourceChangedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_SourceChanged)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_add_Submitting(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_Submitting(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_remove_Submitting(This, eventCookie) \
    ((This)->lpVtbl->remove_Submitting(This, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_add_Completed(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_Completed(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_remove_Completed(This, eventCookie) \
    ((This)->lpVtbl->remove_Completed(This, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_add_SourceChanged(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_SourceChanged(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_remove_SourceChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_SourceChanged(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskCompletedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskCompletedEventArgs[] = L"Windows.Graphics.Printing3D.IPrint3DTaskCompletedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Completion)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrint3DTaskCompletion* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedStatus)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrint3DTaskDetail* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_get_Completion(This, value) \
    ((This)->lpVtbl->get_Completion(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_get_ExtendedStatus(This, value) \
    ((This)->lpVtbl->get_ExtendedStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskRequest
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskRequest
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskRequest[] = L"Windows.Graphics.Printing3D.IPrint3DTaskRequest";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateTask)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest* This,
        HSTRING title,
        HSTRING printerId,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedHandler* handler,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTask** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_CreateTask(This, title, printerId, handler, result) \
    ((This)->lpVtbl->CreateTask(This, title, printerId, handler, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskRequestedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskRequestedEventArgs[] = L"Windows.Graphics.Printing3D.IPrint3DTaskRequestedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskSourceChangedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskSourceChangedEventArgs[] = L"Windows.Graphics.Printing3D.IPrint3DTaskSourceChangedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrint3DTaskSourceRequestedArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrint3DTaskSourceRequestedArgs[] = L"Windows.Graphics.Printing3D.IPrint3DTaskSourceRequestedArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetSource)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* source);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_SetSource(This, source) \
    ((This)->lpVtbl->SetSource(This, source))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrint3DTaskSourceRequestedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3D3MFPackage
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3D3MFPackage
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3D3MFPackage[] = L"Windows.Graphics.Printing3D.IPrinting3D3MFPackage";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation);
    HRESULT (STDMETHODCALLTYPE* get_PrintTicket)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** value);
    HRESULT (STDMETHODCALLTYPE* put_PrintTicket)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value);
    HRESULT (STDMETHODCALLTYPE* get_ModelPart)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** value);
    HRESULT (STDMETHODCALLTYPE* put_ModelPart)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** value);
    HRESULT (STDMETHODCALLTYPE* put_Thumbnail)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* value);
    HRESULT (STDMETHODCALLTYPE* get_Textures)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTextureResource** value);
    HRESULT (STDMETHODCALLTYPE* LoadModelFromPackageAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value,
        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DModel** operation);
    HRESULT (STDMETHODCALLTYPE* SaveModelToPackageAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_SaveAsync(This, operation) \
    ((This)->lpVtbl->SaveAsync(This, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_get_PrintTicket(This, value) \
    ((This)->lpVtbl->get_PrintTicket(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_put_PrintTicket(This, value) \
    ((This)->lpVtbl->put_PrintTicket(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_get_ModelPart(This, value) \
    ((This)->lpVtbl->get_ModelPart(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_put_ModelPart(This, value) \
    ((This)->lpVtbl->put_ModelPart(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_put_Thumbnail(This, value) \
    ((This)->lpVtbl->put_Thumbnail(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_get_Textures(This, value) \
    ((This)->lpVtbl->get_Textures(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_LoadModelFromPackageAsync(This, value, operation) \
    ((This)->lpVtbl->LoadModelFromPackageAsync(This, value, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_SaveModelToPackageAsync(This, value, operation) \
    ((This)->lpVtbl->SaveModelToPackageAsync(This, value, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3D3MFPackage2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3D3MFPackage
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3D3MFPackage2[] = L"Windows.Graphics.Printing3D.IPrinting3D3MFPackage2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Compression)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DPackageCompression* value);
    HRESULT (STDMETHODCALLTYPE* put_Compression)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DPackageCompression value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_get_Compression(This, value) \
    ((This)->lpVtbl->get_Compression(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_put_Compression(This, value) \
    ((This)->lpVtbl->put_Compression(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3D3MFPackageStatics
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3D3MFPackage
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3D3MFPackageStatics[] = L"Windows.Graphics.Printing3D.IPrinting3D3MFPackageStatics";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value,
        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3D3MFPackage** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_LoadAsync(This, value, operation) \
    ((This)->lpVtbl->LoadAsync(This, value, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3D3MFPackageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DBaseMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DBaseMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DBaseMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DBaseMaterial";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Color)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial** value);
    HRESULT (STDMETHODCALLTYPE* put_Color)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_get_Color(This, value) \
    ((This)->lpVtbl->get_Color(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_put_Color(This, value) \
    ((This)->lpVtbl->put_Color(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DBaseMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroup";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Bases)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterial** value);
    HRESULT (STDMETHODCALLTYPE* get_MaterialGroupId)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_get_Bases(This, value) \
    ((This)->lpVtbl->get_Bases(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_get_MaterialGroupId(This, value) \
    ((This)->lpVtbl->get_MaterialGroupId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DBaseMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroupFactory";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory* This,
        UINT32 MaterialGroupId,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_Create(This, MaterialGroupId, result) \
    ((This)->lpVtbl->Create(This, MaterialGroupId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DBaseMaterialStatics
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DBaseMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DBaseMaterialStatics[] = L"Windows.Graphics.Printing3D.IPrinting3DBaseMaterialStatics";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Abs)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Pla)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_get_Abs(This, value) \
    ((This)->lpVtbl->get_Abs(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_get_Pla(This, value) \
    ((This)->lpVtbl->get_Pla(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DColorMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DColorMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DColorMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DColorMaterial";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DColorMaterial2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DColorMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DColorMaterial2[] = L"Windows.Graphics.Printing3D.IPrinting3DColorMaterial2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Color)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_Color)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2* This,
        struct __x_ABI_CWindows_CUI_CColor value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_get_Color(This, value) \
    ((This)->lpVtbl->get_Color(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_put_Color(This, value) \
    ((This)->lpVtbl->put_Color(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterial2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DColorMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DColorMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroup";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Colors)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterial** value);
    HRESULT (STDMETHODCALLTYPE* get_MaterialGroupId)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_get_Colors(This, value) \
    ((This)->lpVtbl->get_Colors(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_get_MaterialGroupId(This, value) \
    ((This)->lpVtbl->get_MaterialGroupId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DColorMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DColorMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroupFactory";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory* This,
        UINT32 MaterialGroupId,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroup** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_Create(This, MaterialGroupId, result) \
    ((This)->lpVtbl->Create(This, MaterialGroupId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DColorMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DComponent
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DComponent
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DComponent[] = L"Windows.Graphics.Printing3D.IPrinting3DComponent";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mesh)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh** value);
    HRESULT (STDMETHODCALLTYPE* put_Mesh)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* value);
    HRESULT (STDMETHODCALLTYPE* get_Components)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponentWithMatrix** value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** value);
    HRESULT (STDMETHODCALLTYPE* put_Thumbnail)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DObjectType* value);
    HRESULT (STDMETHODCALLTYPE* put_Type)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DObjectType value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PartNumber)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PartNumber)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_get_Mesh(This, value) \
    ((This)->lpVtbl->get_Mesh(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_put_Mesh(This, value) \
    ((This)->lpVtbl->put_Mesh(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_get_Components(This, value) \
    ((This)->lpVtbl->get_Components(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_put_Thumbnail(This, value) \
    ((This)->lpVtbl->put_Thumbnail(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_put_Type(This, value) \
    ((This)->lpVtbl->put_Type(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_get_PartNumber(This, value) \
    ((This)->lpVtbl->get_PartNumber(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_put_PartNumber(This, value) \
    ((This)->lpVtbl->put_PartNumber(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DComponentWithMatrix
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DComponentWithMatrix
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DComponentWithMatrix[] = L"Windows.Graphics.Printing3D.IPrinting3DComponentWithMatrix";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrixVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Component)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** value);
    HRESULT (STDMETHODCALLTYPE* put_Component)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* value);
    HRESULT (STDMETHODCALLTYPE* get_Matrix)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4* value);
    HRESULT (STDMETHODCALLTYPE* put_Matrix)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix4x4 value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrixVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrixVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_get_Component(This, value) \
    ((This)->lpVtbl->get_Component(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_put_Component(This, value) \
    ((This)->lpVtbl->put_Component(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_get_Matrix(This, value) \
    ((This)->lpVtbl->get_Matrix(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_put_Matrix(This, value) \
    ((This)->lpVtbl->put_Matrix(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponentWithMatrix_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DCompositeMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DCompositeMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DCompositeMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DCompositeMaterial";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Values)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial* This,
        __FIVector_1_double** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_get_Values(This, value) \
    ((This)->lpVtbl->get_Values(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DCompositeMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Composites)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterial** value);
    HRESULT (STDMETHODCALLTYPE* get_MaterialGroupId)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaterialIndices)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup* This,
        __FIVector_1_UINT32** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_get_Composites(This, value) \
    ((This)->lpVtbl->get_Composites(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_get_MaterialGroupId(This, value) \
    ((This)->lpVtbl->get_MaterialGroupId(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_get_MaterialIndices(This, value) \
    ((This)->lpVtbl->get_MaterialIndices(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DCompositeMaterialGroup2[] = L"Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BaseMaterialGroup)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup** value);
    HRESULT (STDMETHODCALLTYPE* put_BaseMaterialGroup)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DBaseMaterialGroup* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_get_BaseMaterialGroup(This, value) \
    ((This)->lpVtbl->get_BaseMaterialGroup(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_put_BaseMaterialGroup(This, value) \
    ((This)->lpVtbl->put_BaseMaterialGroup(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DCompositeMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroupFactory";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory* This,
        UINT32 MaterialGroupId,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroup** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_Create(This, MaterialGroupId, result) \
    ((This)->lpVtbl->Create(This, MaterialGroupId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DCompositeMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DFaceReductionOptions
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DFaceReductionOptions
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DFaceReductionOptions[] = L"Windows.Graphics.Printing3D.IPrinting3DFaceReductionOptions";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxReductionArea)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxReductionArea)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_TargetTriangleCount)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TargetTriangleCount)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_MaxEdgeLength)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxEdgeLength)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptionsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_get_MaxReductionArea(This, value) \
    ((This)->lpVtbl->get_MaxReductionArea(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_put_MaxReductionArea(This, value) \
    ((This)->lpVtbl->put_MaxReductionArea(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_get_TargetTriangleCount(This, value) \
    ((This)->lpVtbl->get_TargetTriangleCount(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_put_TargetTriangleCount(This, value) \
    ((This)->lpVtbl->put_TargetTriangleCount(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_get_MaxEdgeLength(This, value) \
    ((This)->lpVtbl->get_MaxEdgeLength(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_put_MaxEdgeLength(This, value) \
    ((This)->lpVtbl->put_MaxEdgeLength(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DMaterial";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BaseGroups)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DBaseMaterialGroup** value);
    HRESULT (STDMETHODCALLTYPE* get_ColorGroups)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DColorMaterialGroup** value);
    HRESULT (STDMETHODCALLTYPE* get_Texture2CoordGroups)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterialGroup** value);
    HRESULT (STDMETHODCALLTYPE* get_CompositeGroups)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DCompositeMaterialGroup** value);
    HRESULT (STDMETHODCALLTYPE* get_MultiplePropertyGroups)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterialGroup** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterialVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_get_BaseGroups(This, value) \
    ((This)->lpVtbl->get_BaseGroups(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_get_ColorGroups(This, value) \
    ((This)->lpVtbl->get_ColorGroups(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_get_Texture2CoordGroups(This, value) \
    ((This)->lpVtbl->get_Texture2CoordGroups(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_get_CompositeGroups(This, value) \
    ((This)->lpVtbl->get_CompositeGroups(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_get_MultiplePropertyGroups(This, value) \
    ((This)->lpVtbl->get_MultiplePropertyGroups(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMesh
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMesh
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMesh[] = L"Windows.Graphics.Printing3D.IPrinting3DMesh";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VertexCount)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_VertexCount)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_IndexCount)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_IndexCount)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_VertexPositionsDescription)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription* value);
    HRESULT (STDMETHODCALLTYPE* put_VertexPositionsDescription)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription value);
    HRESULT (STDMETHODCALLTYPE* get_VertexNormalsDescription)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription* value);
    HRESULT (STDMETHODCALLTYPE* put_VertexNormalsDescription)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription value);
    HRESULT (STDMETHODCALLTYPE* get_TriangleIndicesDescription)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription* value);
    HRESULT (STDMETHODCALLTYPE* put_TriangleIndicesDescription)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription value);
    HRESULT (STDMETHODCALLTYPE* get_TriangleMaterialIndicesDescription)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription* value);
    HRESULT (STDMETHODCALLTYPE* put_TriangleMaterialIndicesDescription)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        struct __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DBufferDescription value);
    HRESULT (STDMETHODCALLTYPE* GetVertexPositions)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* CreateVertexPositions)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* GetVertexNormals)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* CreateVertexNormals)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* GetTriangleIndices)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* CreateTriangleIndices)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* GetTriangleMaterialIndices)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** buffer);
    HRESULT (STDMETHODCALLTYPE* CreateTriangleMaterialIndices)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BufferDescriptionSet)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* get_BufferSet)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* VerifyAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DMeshVerificationMode value,
        __FIAsyncOperation_1_Windows__CGraphics__CPrinting3D__CPrinting3DMeshVerificationResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_get_VertexCount(This, value) \
    ((This)->lpVtbl->get_VertexCount(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_put_VertexCount(This, value) \
    ((This)->lpVtbl->put_VertexCount(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_get_IndexCount(This, value) \
    ((This)->lpVtbl->get_IndexCount(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_put_IndexCount(This, value) \
    ((This)->lpVtbl->put_IndexCount(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_get_VertexPositionsDescription(This, value) \
    ((This)->lpVtbl->get_VertexPositionsDescription(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_put_VertexPositionsDescription(This, value) \
    ((This)->lpVtbl->put_VertexPositionsDescription(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_get_VertexNormalsDescription(This, value) \
    ((This)->lpVtbl->get_VertexNormalsDescription(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_put_VertexNormalsDescription(This, value) \
    ((This)->lpVtbl->put_VertexNormalsDescription(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_get_TriangleIndicesDescription(This, value) \
    ((This)->lpVtbl->get_TriangleIndicesDescription(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_put_TriangleIndicesDescription(This, value) \
    ((This)->lpVtbl->put_TriangleIndicesDescription(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_get_TriangleMaterialIndicesDescription(This, value) \
    ((This)->lpVtbl->get_TriangleMaterialIndicesDescription(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_put_TriangleMaterialIndicesDescription(This, value) \
    ((This)->lpVtbl->put_TriangleMaterialIndicesDescription(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_GetVertexPositions(This, buffer) \
    ((This)->lpVtbl->GetVertexPositions(This, buffer))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_CreateVertexPositions(This, value) \
    ((This)->lpVtbl->CreateVertexPositions(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_GetVertexNormals(This, buffer) \
    ((This)->lpVtbl->GetVertexNormals(This, buffer))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_CreateVertexNormals(This, value) \
    ((This)->lpVtbl->CreateVertexNormals(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_GetTriangleIndices(This, buffer) \
    ((This)->lpVtbl->GetTriangleIndices(This, buffer))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_CreateTriangleIndices(This, value) \
    ((This)->lpVtbl->CreateTriangleIndices(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_GetTriangleMaterialIndices(This, buffer) \
    ((This)->lpVtbl->GetTriangleMaterialIndices(This, buffer))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_CreateTriangleMaterialIndices(This, value) \
    ((This)->lpVtbl->CreateTriangleMaterialIndices(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_get_BufferDescriptionSet(This, value) \
    ((This)->lpVtbl->get_BufferDescriptionSet(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_get_BufferSet(This, value) \
    ((This)->lpVtbl->get_BufferSet(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_VerifyAsync(This, value, operation) \
    ((This)->lpVtbl->VerifyAsync(This, value, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMesh_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMeshVerificationResult
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMeshVerificationResult
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMeshVerificationResult[] = L"Windows.Graphics.Printing3D.IPrinting3DMeshVerificationResult";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsValid)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_NonmanifoldTriangles)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This,
        __FIVectorView_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_ReversedNormalTriangles)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult* This,
        __FIVectorView_1_UINT32** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResultVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_get_IsValid(This, value) \
    ((This)->lpVtbl->get_IsValid(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_get_NonmanifoldTriangles(This, value) \
    ((This)->lpVtbl->get_NonmanifoldTriangles(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_get_ReversedNormalTriangles(This, value) \
    ((This)->lpVtbl->get_ReversedNormalTriangles(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMeshVerificationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DModel
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DModel
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DModel[] = L"Windows.Graphics.Printing3D.IPrinting3DModel";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Unit)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DModelUnit* value);
    HRESULT (STDMETHODCALLTYPE* put_Unit)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DModelUnit value);
    HRESULT (STDMETHODCALLTYPE* get_Textures)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DModelTexture** value);
    HRESULT (STDMETHODCALLTYPE* get_Meshes)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMesh** value);
    HRESULT (STDMETHODCALLTYPE* get_Components)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DComponent** value);
    HRESULT (STDMETHODCALLTYPE* get_Material)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial** value);
    HRESULT (STDMETHODCALLTYPE* put_Material)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMaterial* value);
    HRESULT (STDMETHODCALLTYPE* get_Build)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent** value);
    HRESULT (STDMETHODCALLTYPE* put_Build)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DComponent* value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Version)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_RequiredExtensions)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Metadata)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* RepairAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_Unit(This, value) \
    ((This)->lpVtbl->get_Unit(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_put_Unit(This, value) \
    ((This)->lpVtbl->put_Unit(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_Textures(This, value) \
    ((This)->lpVtbl->get_Textures(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_Meshes(This, value) \
    ((This)->lpVtbl->get_Meshes(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_Components(This, value) \
    ((This)->lpVtbl->get_Components(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_Material(This, value) \
    ((This)->lpVtbl->get_Material(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_put_Material(This, value) \
    ((This)->lpVtbl->put_Material(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_Build(This, value) \
    ((This)->lpVtbl->get_Build(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_put_Build(This, value) \
    ((This)->lpVtbl->put_Build(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_put_Version(This, value) \
    ((This)->lpVtbl->put_Version(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_RequiredExtensions(This, value) \
    ((This)->lpVtbl->get_RequiredExtensions(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_get_Metadata(This, value) \
    ((This)->lpVtbl->get_Metadata(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_RepairAsync(This, operation) \
    ((This)->lpVtbl->RepairAsync(This, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_Clone(This, value) \
    ((This)->lpVtbl->Clone(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DModel2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DModel
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DModel2[] = L"Windows.Graphics.Printing3D.IPrinting3DModel2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryPartialRepairAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryPartialRepairWithTimeAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan maxWaitTime,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryReduceFacesAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        __FIAsyncOperationWithProgress_2_boolean_double** operation);
    HRESULT (STDMETHODCALLTYPE* TryReduceFacesWithOptionsAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* printing3DFaceReductionOptions,
        __FIAsyncOperationWithProgress_2_boolean_double** operation);
    HRESULT (STDMETHODCALLTYPE* TryReduceFacesWithOptionsAndTimeAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DFaceReductionOptions* printing3DFaceReductionOptions,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan maxWait,
        __FIAsyncOperationWithProgress_2_boolean_double** operation);
    HRESULT (STDMETHODCALLTYPE* RepairWithProgressAsync)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2* This,
        __FIAsyncOperationWithProgress_2_boolean_double** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_TryPartialRepairAsync(This, operation) \
    ((This)->lpVtbl->TryPartialRepairAsync(This, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_TryPartialRepairWithTimeAsync(This, maxWaitTime, operation) \
    ((This)->lpVtbl->TryPartialRepairWithTimeAsync(This, maxWaitTime, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_TryReduceFacesAsync(This, operation) \
    ((This)->lpVtbl->TryReduceFacesAsync(This, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_TryReduceFacesWithOptionsAsync(This, printing3DFaceReductionOptions, operation) \
    ((This)->lpVtbl->TryReduceFacesWithOptionsAsync(This, printing3DFaceReductionOptions, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_TryReduceFacesWithOptionsAndTimeAsync(This, printing3DFaceReductionOptions, maxWait, operation) \
    ((This)->lpVtbl->TryReduceFacesWithOptionsAndTimeAsync(This, printing3DFaceReductionOptions, maxWait, operation))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_RepairWithProgressAsync(This, operation) \
    ((This)->lpVtbl->RepairWithProgressAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModel2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DModelTexture
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DModelTexture
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DModelTexture[] = L"Windows.Graphics.Printing3D.IPrinting3DModelTexture";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTextureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextureResource)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource** value);
    HRESULT (STDMETHODCALLTYPE* put_TextureResource)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* value);
    HRESULT (STDMETHODCALLTYPE* get_TileStyleU)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DTextureEdgeBehavior* value);
    HRESULT (STDMETHODCALLTYPE* put_TileStyleU)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DTextureEdgeBehavior value);
    HRESULT (STDMETHODCALLTYPE* get_TileStyleV)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DTextureEdgeBehavior* value);
    HRESULT (STDMETHODCALLTYPE* put_TileStyleV)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting3D_CPrinting3DTextureEdgeBehavior value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTextureVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTextureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_get_TextureResource(This, value) \
    ((This)->lpVtbl->get_TextureResource(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_put_TextureResource(This, value) \
    ((This)->lpVtbl->put_TextureResource(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_get_TileStyleU(This, value) \
    ((This)->lpVtbl->get_TileStyleU(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_put_TileStyleU(This, value) \
    ((This)->lpVtbl->put_TileStyleU(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_get_TileStyleV(This, value) \
    ((This)->lpVtbl->get_TileStyleV(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_put_TileStyleV(This, value) \
    ((This)->lpVtbl->put_TileStyleV(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMultiplePropertyMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterial";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaterialIndices)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial* This,
        __FIVector_1_UINT32** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_get_MaterialIndices(This, value) \
    ((This)->lpVtbl->get_MaterialIndices(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMultiplePropertyMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroup";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MultipleProperties)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DMultiplePropertyMaterial** value);
    HRESULT (STDMETHODCALLTYPE* get_MaterialGroupIndices)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This,
        __FIVector_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_MaterialGroupId)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_get_MultipleProperties(This, value) \
    ((This)->lpVtbl->get_MultipleProperties(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_get_MaterialGroupIndices(This, value) \
    ((This)->lpVtbl->get_MaterialGroupIndices(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_get_MaterialGroupId(This, value) \
    ((This)->lpVtbl->get_MaterialGroupId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DMultiplePropertyMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroupFactory";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory* This,
        UINT32 MaterialGroupId,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroup** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_Create(This, MaterialGroupId, result) \
    ((This)->lpVtbl->Create(This, MaterialGroupId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DMultiplePropertyMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTexture2CoordMaterial[] = L"Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterial";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Texture)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** value);
    HRESULT (STDMETHODCALLTYPE* put_Texture)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* value);
    HRESULT (STDMETHODCALLTYPE* get_U)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_U)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_V)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_V)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_get_Texture(This, value) \
    ((This)->lpVtbl->get_Texture(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_put_Texture(This, value) \
    ((This)->lpVtbl->put_Texture(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_get_U(This, value) \
    ((This)->lpVtbl->get_U(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_put_U(This, value) \
    ((This)->lpVtbl->put_U(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_get_V(This, value) \
    ((This)->lpVtbl->get_V(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_put_V(This, value) \
    ((This)->lpVtbl->put_V(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTexture2CoordMaterialGroup[] = L"Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Texture2Coords)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* This,
        __FIVector_1_Windows__CGraphics__CPrinting3D__CPrinting3DTexture2CoordMaterial** value);
    HRESULT (STDMETHODCALLTYPE* get_MaterialGroupId)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_get_Texture2Coords(This, value) \
    ((This)->lpVtbl->get_Texture2Coords(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_get_MaterialGroupId(This, value) \
    ((This)->lpVtbl->get_MaterialGroupId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup2
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTexture2CoordMaterialGroup2[] = L"Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Texture)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture** value);
    HRESULT (STDMETHODCALLTYPE* put_Texture)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2* This,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DModelTexture* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_get_Texture(This, value) \
    ((This)->lpVtbl->get_Texture(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_put_Texture(This, value) \
    ((This)->lpVtbl->put_Texture(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup2_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroupFactory
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTexture2CoordMaterialGroupFactory[] = L"Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroupFactory";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory* This,
        UINT32 MaterialGroupId,
        __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroup** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactoryVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_Create(This, MaterialGroupId, result) \
    ((This)->lpVtbl->Create(This, MaterialGroupId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTexture2CoordMaterialGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing3D.IPrinting3DTextureResource
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing3D.Printing3DTextureResource
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing3D_IPrinting3DTextureResource[] = L"Windows.Graphics.Printing3D.IPrinting3DTextureResource";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextureData)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** value);
    HRESULT (STDMETHODCALLTYPE* put_TextureData)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResourceVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_get_TextureData(This, value) \
    ((This)->lpVtbl->get_TextureData(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_put_TextureData(This, value) \
    ((This)->lpVtbl->put_TextureData(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting3D_CIPrinting3DTextureResource_INTERFACE_DEFINED__) */
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DManager
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing3D.IPrint3DManagerStatics interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DManager_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DManager[] = L"Windows.Graphics.Printing3D.Print3DManager";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTask
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTask ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTask_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTask[] = L"Windows.Graphics.Printing3D.Print3DTask";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskCompletedEventArgs[] = L"Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskRequest
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskRequest[] = L"Windows.Graphics.Printing3D.Print3DTaskRequest";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskRequestedEventArgs[] = L"Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskSourceChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskSourceChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskSourceChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskSourceChangedEventArgs[] = L"Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrint3DTaskSourceRequestedArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskSourceRequestedArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Print3DTaskSourceRequestedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Print3DTaskSourceRequestedArgs[] = L"Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3D3MFPackage
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing3D.IPrinting3D3MFPackageStatics interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3D3MFPackage ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3D3MFPackage2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3D3MFPackage_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3D3MFPackage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3D3MFPackage[] = L"Windows.Graphics.Printing3D.Printing3D3MFPackage";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DBaseMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing3D.IPrinting3DBaseMaterialStatics interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DBaseMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DBaseMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DBaseMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DBaseMaterial[] = L"Windows.Graphics.Printing3D.Printing3DBaseMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DBaseMaterialGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DBaseMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DBaseMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DBaseMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DColorMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DColorMaterial ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3DColorMaterial2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DColorMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DColorMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DColorMaterial[] = L"Windows.Graphics.Printing3D.Printing3DColorMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DColorMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DColorMaterialGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DColorMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DColorMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DColorMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DColorMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DComponent
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DComponent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DComponent_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DComponent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DComponent[] = L"Windows.Graphics.Printing3D.Printing3DComponent";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DComponentWithMatrix
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DComponentWithMatrix ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DComponentWithMatrix_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DComponentWithMatrix_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DComponentWithMatrix[] = L"Windows.Graphics.Printing3D.Printing3DComponentWithMatrix";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DCompositeMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DCompositeMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DCompositeMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DCompositeMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DCompositeMaterial[] = L"Windows.Graphics.Printing3D.Printing3DCompositeMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3DCompositeMaterialGroup2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DCompositeMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DCompositeMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DCompositeMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DFaceReductionOptions
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DFaceReductionOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DFaceReductionOptions_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DFaceReductionOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DFaceReductionOptions[] = L"Windows.Graphics.Printing3D.Printing3DFaceReductionOptions";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMaterial[] = L"Windows.Graphics.Printing3D.Printing3DMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMesh
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMesh ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMesh_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMesh_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMesh[] = L"Windows.Graphics.Printing3D.Printing3DMesh";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMeshVerificationResult
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMeshVerificationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMeshVerificationResult_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMeshVerificationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMeshVerificationResult[] = L"Windows.Graphics.Printing3D.Printing3DMeshVerificationResult";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DModel
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DModel ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3DModel2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DModel_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DModel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DModel[] = L"Windows.Graphics.Printing3D.Printing3DModel";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DModelTexture
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DModelTexture ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DModelTexture_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DModelTexture_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DModelTexture[] = L"Windows.Graphics.Printing3D.Printing3DModelTexture";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterial[] = L"Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DMultiplePropertyMaterialGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DMultiplePropertyMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterial_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterial[] = L"Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroupFactory interface starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup ** Default Interface **
 *    Windows.Graphics.Printing3D.IPrinting3DTexture2CoordMaterialGroup2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterialGroup_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterialGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DTexture2CoordMaterialGroup[] = L"Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing3D.Printing3DTextureResource
 *
 * Introduced to Windows.Graphics.Printing3D.Printing3DContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Graphics.Printing3D.Printing3DContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing3D.IPrinting3DTextureResource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTextureResource_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing3D_Printing3DTextureResource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing3D_Printing3DTextureResource[] = L"Windows.Graphics.Printing3D.Printing3DTextureResource";
#endif
#endif // WINDOWS_GRAPHICS_PRINTING3D_PRINTING3DCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egraphics2Eprinting3d_p_h__

#endif // __windows2Egraphics2Eprinting3d_h__
