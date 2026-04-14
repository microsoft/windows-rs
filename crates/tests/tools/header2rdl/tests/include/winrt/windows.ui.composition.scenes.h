
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
#ifndef __windows2Eui2Ecomposition2Escenes_h__
#define __windows2Eui2Ecomposition2Escenes_h__
#ifndef __windows2Eui2Ecomposition2Escenes_p_h__
#define __windows2Eui2Ecomposition2Escenes_p_h__


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
#include "Windows.Foundation.Numerics.h"
#include "Windows.Graphics.DirectX.h"
#include "Windows.UI.Composition.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneBoundingBox;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox ABI::Windows::UI::Composition::Scenes::ISceneBoundingBox

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneComponent;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent ABI::Windows::UI::Composition::Scenes::ISceneComponent

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneComponentCollection;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection ABI::Windows::UI::Composition::Scenes::ISceneComponentCollection

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneComponentFactory;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory ABI::Windows::UI::Composition::Scenes::ISceneComponentFactory

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMaterial;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial ABI::Windows::UI::Composition::Scenes::ISceneMaterial

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMaterialFactory;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory ABI::Windows::UI::Composition::Scenes::ISceneMaterialFactory

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMaterialInput;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMaterialInputFactory;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory ABI::Windows::UI::Composition::Scenes::ISceneMaterialInputFactory

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMesh;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh ABI::Windows::UI::Composition::Scenes::ISceneMesh

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMeshMaterialAttributeMap;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap ABI::Windows::UI::Composition::Scenes::ISceneMeshMaterialAttributeMap

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMeshRendererComponent;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent ABI::Windows::UI::Composition::Scenes::ISceneMeshRendererComponent

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMeshRendererComponentStatics;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics ABI::Windows::UI::Composition::Scenes::ISceneMeshRendererComponentStatics

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMeshStatics;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics ABI::Windows::UI::Composition::Scenes::ISceneMeshStatics

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMetallicRoughnessMaterial;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial ABI::Windows::UI::Composition::Scenes::ISceneMetallicRoughnessMaterial

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneMetallicRoughnessMaterialStatics;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics ABI::Windows::UI::Composition::Scenes::ISceneMetallicRoughnessMaterialStatics

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneModelTransform;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform ABI::Windows::UI::Composition::Scenes::ISceneModelTransform

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneNode;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode ABI::Windows::UI::Composition::Scenes::ISceneNode

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneNodeCollection;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection ABI::Windows::UI::Composition::Scenes::ISceneNodeCollection

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneNodeStatics;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics ABI::Windows::UI::Composition::Scenes::ISceneNodeStatics

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneObject;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject ABI::Windows::UI::Composition::Scenes::ISceneObject

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneObjectFactory;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory ABI::Windows::UI::Composition::Scenes::ISceneObjectFactory

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface IScenePbrMaterial;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial ABI::Windows::UI::Composition::Scenes::IScenePbrMaterial

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface IScenePbrMaterialFactory;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory ABI::Windows::UI::Composition::Scenes::IScenePbrMaterialFactory

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneRendererComponent;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent ABI::Windows::UI::Composition::Scenes::ISceneRendererComponent

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneRendererComponentFactory;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory ABI::Windows::UI::Composition::Scenes::ISceneRendererComponentFactory

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneSurfaceMaterialInput;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput ABI::Windows::UI::Composition::Scenes::ISceneSurfaceMaterialInput

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneSurfaceMaterialInputStatics;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics ABI::Windows::UI::Composition::Scenes::ISceneSurfaceMaterialInputStatics

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneVisual;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual ABI::Windows::UI::Composition::Scenes::ISceneVisual

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    interface ISceneVisualStatics;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics ABI::Windows::UI::Composition::Scenes::ISceneVisualStatics

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    typedef enum SceneAttributeSemantic : int SceneAttributeSemantic;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("55ef41a2-86b4-5d95-9d2c-747f340779a9"))
IKeyValuePair<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic> : IKeyValuePair_impl<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic> __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6e8c9202-4878-50ae-878e-48a303caf1f8"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a3e30221-7ca2-5a3c-a54a-378fee7369cc"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneComponent;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE
#define DEF___FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c819527f-80b2-5b44-a3a2-368c1d359f8f"))
IIterator<ABI::Windows::UI::Composition::Scenes::SceneComponent*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Composition::Scenes::SceneComponent*, ABI::Windows::UI::Composition::Scenes::ISceneComponent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Composition.Scenes.SceneComponent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Composition::Scenes::SceneComponent*> __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_t;
#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE
#define DEF___FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9c5148db-05a5-505a-bb14-a0e5dfbb2cd4"))
IIterable<ABI::Windows::UI::Composition::Scenes::SceneComponent*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Composition::Scenes::SceneComponent*, ABI::Windows::UI::Composition::Scenes::ISceneComponent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Composition.Scenes.SceneComponent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Composition::Scenes::SceneComponent*> __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_t;
#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneNode;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE
#define DEF___FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b6d9261d-6a3a-5ca4-bf99-9b7efe4d0f88"))
IIterator<ABI::Windows::UI::Composition::Scenes::SceneNode*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Composition::Scenes::SceneNode*, ABI::Windows::UI::Composition::Scenes::ISceneNode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Composition.Scenes.SceneNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Composition::Scenes::SceneNode*> __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_t;
#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE
#define DEF___FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("db4730e4-f364-576b-878e-59a7c459a752"))
IIterable<ABI::Windows::UI::Composition::Scenes::SceneNode*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Composition::Scenes::SceneNode*, ABI::Windows::UI::Composition::Scenes::ISceneNode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Composition.Scenes.SceneNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Composition::Scenes::SceneNode*> __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_t;
#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#define DEF___FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("10693bb4-d94c-5b35-8aea-f43df6603800"))
IMapView<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic> : IMapView_impl<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic> __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t;
#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#define DEF___FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("41f50f14-9a3c-5240-b042-1bff02e57028"))
IMap<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic> : IMap_impl<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, enum ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic> __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t;
#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE
#define DEF___FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("17cceac1-fe0a-535d-91d3-a53431e03ed2"))
IVectorView<ABI::Windows::UI::Composition::Scenes::SceneComponent*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Composition::Scenes::SceneComponent*, ABI::Windows::UI::Composition::Scenes::ISceneComponent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Composition.Scenes.SceneComponent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Composition::Scenes::SceneComponent*> __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_t;
#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE
#define DEF___FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b5871458-c28c-50c9-acd8-76e7871937fb"))
IVectorView<ABI::Windows::UI::Composition::Scenes::SceneNode*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Composition::Scenes::SceneNode*, ABI::Windows::UI::Composition::Scenes::ISceneNode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Composition.Scenes.SceneNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Composition::Scenes::SceneNode*> __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_t;
#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE
#define DEF___FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("daad0f31-c450-5060-9732-f02c885e9b3f"))
IVector<ABI::Windows::UI::Composition::Scenes::SceneComponent*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Composition::Scenes::SceneComponent*, ABI::Windows::UI::Composition::Scenes::ISceneComponent*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Composition.Scenes.SceneComponent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Composition::Scenes::SceneComponent*> __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_t;
#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE
#define DEF___FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("732596a0-6d36-5a59-8a0b-8ff16142b893"))
IVector<ABI::Windows::UI::Composition::Scenes::SceneNode*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Composition::Scenes::SceneNode*, ABI::Windows::UI::Composition::Scenes::ISceneNode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Composition.Scenes.SceneNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Composition::Scenes::SceneNode*> __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_t;
#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class MemoryBuffer;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
                typedef struct Vector3 Vector3;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Vector4 Vector4;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            namespace DirectX {
                typedef enum DirectXPrimitiveTopology : int DirectXPrimitiveTopology;
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                typedef enum CompositionBitmapInterpolationMode : int CompositionBitmapInterpolationMode;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                class Compositor;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface ICompositor;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CICompositor ABI::Windows::UI::Composition::ICompositor

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositionSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositionSurface_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface ICompositionSurface;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CICompositionSurface ABI::Windows::UI::Composition::ICompositionSurface

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositionSurface_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    typedef enum SceneAlphaMode : int SceneAlphaMode;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    typedef enum SceneComponentType : int SceneComponentType;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    typedef enum SceneWrappingMode : int SceneWrappingMode;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneBoundingBox;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneComponentCollection;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneMaterial;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneMaterialInput;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneMesh;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneMeshMaterialAttributeMap;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneMeshRendererComponent;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneMetallicRoughnessMaterial;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneModelTransform;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneNodeCollection;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneSurfaceMaterialInput;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    class SceneVisual;
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Composition.Scenes.SceneAlphaMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    enum SceneAlphaMode : int
                    {
                        SceneAlphaMode_Opaque = 0,
                        SceneAlphaMode_AlphaTest = 1,
                        SceneAlphaMode_Blend = 2,
                    };
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Composition.Scenes.SceneAttributeSemantic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    enum SceneAttributeSemantic : int
                    {
                        SceneAttributeSemantic_Index = 0,
                        SceneAttributeSemantic_Vertex = 1,
                        SceneAttributeSemantic_Normal = 2,
                        SceneAttributeSemantic_TexCoord0 = 3,
                        SceneAttributeSemantic_TexCoord1 = 4,
                        SceneAttributeSemantic_Color = 5,
                        SceneAttributeSemantic_Tangent = 6,
                    };
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Composition.Scenes.SceneComponentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    enum SceneComponentType : int
                    {
                        SceneComponentType_MeshRendererComponent = 0,
                    };
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Composition.Scenes.SceneWrappingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    enum SceneWrappingMode : int
                    {
                        SceneWrappingMode_ClampToEdge = 0,
                        SceneWrappingMode_MirroredRepeat = 1,
                        SceneWrappingMode_Repeat = 2,
                    };
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneBoundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneBoundingBox
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneBoundingBox[] = L"Windows.UI.Composition.Scenes.ISceneBoundingBox";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("5d8ffc70-c618-4083-8251-9962593114aa")
                    ISceneBoundingBox : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Center(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Extents(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Max(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Min(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Size(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneBoundingBox = __uuidof(ISceneBoundingBox);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneComponent[] = L"Windows.UI.Composition.Scenes.ISceneComponent";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("ae20fc96-226c-44bd-95cb-dd5ed9ebe9a5")
                    ISceneComponent : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ComponentType(
                            ABI::Windows::UI::Composition::Scenes::SceneComponentType* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneComponent = __uuidof(ISceneComponent);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneComponentCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneComponentCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneComponentCollection[] = L"Windows.UI.Composition.Scenes.ISceneComponentCollection";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("c483791c-5f46-45e4-b666-a3d2259f9b2e")
                    ISceneComponentCollection : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneComponentCollection = __uuidof(ISceneComponentCollection);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneComponentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneComponentFactory[] = L"Windows.UI.Composition.Scenes.ISceneComponentFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("5fbc5574-ddd8-5889-ab5b-d8fa716e7c9e")
                    ISceneComponentFactory : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneComponentFactory = __uuidof(ISceneComponentFactory);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMaterial[] = L"Windows.UI.Composition.Scenes.ISceneMaterial";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("8ca74b7c-30df-4e07-9490-37875af1a123")
                    ISceneMaterial : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneMaterial = __uuidof(ISceneMaterial);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMaterialFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMaterialFactory[] = L"Windows.UI.Composition.Scenes.ISceneMaterialFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("67536c19-a707-5254-a495-7fdc799893b9")
                    ISceneMaterialFactory : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneMaterialFactory = __uuidof(ISceneMaterialFactory);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMaterialInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMaterialInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMaterialInput[] = L"Windows.UI.Composition.Scenes.ISceneMaterialInput";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("422a1642-1ef1-485c-97e9-ae6f95ad812f")
                    ISceneMaterialInput : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneMaterialInput = __uuidof(ISceneMaterialInput);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMaterialInputFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMaterialInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMaterialInputFactory[] = L"Windows.UI.Composition.Scenes.ISceneMaterialInputFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("a88feb74-7d0a-5e4c-a748-1015af9ca74f")
                    ISceneMaterialInputFactory : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneMaterialInputFactory = __uuidof(ISceneMaterialInputFactory);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMesh
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMesh
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMesh[] = L"Windows.UI.Composition.Scenes.ISceneMesh";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("ee9a1530-1155-4c0c-92bd-40020cf78347")
                    ISceneMesh : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Bounds(
                            ABI::Windows::UI::Composition::Scenes::ISceneBoundingBox** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PrimitiveTopology(
                            ABI::Windows::Graphics::DirectX::DirectXPrimitiveTopology* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PrimitiveTopology(
                            ABI::Windows::Graphics::DirectX::DirectXPrimitiveTopology value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FillMeshAttribute(
                            ABI::Windows::UI::Composition::Scenes::SceneAttributeSemantic semantic,
                            ABI::Windows::Graphics::DirectX::DirectXPixelFormat format,
                            ABI::Windows::Foundation::IMemoryBuffer* memory
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneMesh = __uuidof(ISceneMesh);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMeshMaterialAttributeMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMeshMaterialAttributeMap[] = L"Windows.UI.Composition.Scenes.ISceneMeshMaterialAttributeMap";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("ce843171-3d43-4855-aa69-31ff988d049d")
                    ISceneMeshMaterialAttributeMap : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneMeshMaterialAttributeMap = __uuidof(ISceneMeshMaterialAttributeMap);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMeshRendererComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMeshRendererComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMeshRendererComponent[] = L"Windows.UI.Composition.Scenes.ISceneMeshRendererComponent";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("9929f7e3-6364-477e-98fe-74ed9fd4c2de")
                    ISceneMeshRendererComponent : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Material(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterial** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Material(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterial* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Mesh(
                            ABI::Windows::UI::Composition::Scenes::ISceneMesh** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Mesh(
                            ABI::Windows::UI::Composition::Scenes::ISceneMesh* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UVMappings(
                            ABI::Windows::UI::Composition::Scenes::ISceneMeshMaterialAttributeMap** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneMeshRendererComponent = __uuidof(ISceneMeshRendererComponent);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMeshRendererComponentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMeshRendererComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMeshRendererComponentStatics[] = L"Windows.UI.Composition.Scenes.ISceneMeshRendererComponentStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("4954f37a-4459-4521-bd6e-2b38b8d711ea")
                    ISceneMeshRendererComponentStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Composition::ICompositor* compositor,
                            ABI::Windows::UI::Composition::Scenes::ISceneMeshRendererComponent** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneMeshRendererComponentStatics = __uuidof(ISceneMeshRendererComponentStatics);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMeshStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMesh
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMeshStatics[] = L"Windows.UI.Composition.Scenes.ISceneMeshStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("8412316c-7b57-473f-966b-81dc277b1751")
                    ISceneMeshStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Composition::ICompositor* compositor,
                            ABI::Windows::UI::Composition::Scenes::ISceneMesh** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneMeshStatics = __uuidof(ISceneMeshStatics);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMetallicRoughnessMaterial[] = L"Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterial";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("c1d91446-799c-429e-a4e4-5da645f18e61")
                    ISceneMetallicRoughnessMaterial : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BaseColorInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_BaseColorInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BaseColorFactor(
                            ABI::Windows::Foundation::Numerics::Vector4* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_BaseColorFactor(
                            ABI::Windows::Foundation::Numerics::Vector4 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MetallicFactor(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MetallicFactor(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MetallicRoughnessInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MetallicRoughnessInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RoughnessFactor(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RoughnessFactor(
                            FLOAT value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneMetallicRoughnessMaterial = __uuidof(ISceneMetallicRoughnessMaterial);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterialStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMetallicRoughnessMaterialStatics[] = L"Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterialStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("3bddca50-6d9d-4531-8dc4-b27e3e49b7ab")
                    ISceneMetallicRoughnessMaterialStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Composition::ICompositor* compositor,
                            ABI::Windows::UI::Composition::Scenes::ISceneMetallicRoughnessMaterial** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneMetallicRoughnessMaterialStatics = __uuidof(ISceneMetallicRoughnessMaterialStatics);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneModelTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneModelTransform
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneModelTransform[] = L"Windows.UI.Composition.Scenes.ISceneModelTransform";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("c05576c2-32b1-4269-980d-b98537100ae4")
                    ISceneModelTransform : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                            ABI::Windows::Foundation::Numerics::Quaternion* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Orientation(
                            ABI::Windows::Foundation::Numerics::Quaternion value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RotationAngle(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RotationAngle(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RotationAngleInDegrees(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RotationAngleInDegrees(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RotationAxis(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RotationAxis(
                            ABI::Windows::Foundation::Numerics::Vector3 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Scale(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Scale(
                            ABI::Windows::Foundation::Numerics::Vector3 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Translation(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Translation(
                            ABI::Windows::Foundation::Numerics::Vector3 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneModelTransform = __uuidof(ISceneModelTransform);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneNode[] = L"Windows.UI.Composition.Scenes.ISceneNode";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("acf2c247-f307-4581-9c41-af2e29c3b016")
                    ISceneNode : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Children(
                            __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Components(
                            __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Parent(
                            ABI::Windows::UI::Composition::Scenes::ISceneNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Transform(
                            ABI::Windows::UI::Composition::Scenes::ISceneModelTransform** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FindFirstComponentOfType(
                            ABI::Windows::UI::Composition::Scenes::SceneComponentType value,
                            ABI::Windows::UI::Composition::Scenes::ISceneComponent** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneNode = __uuidof(ISceneNode);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneNodeCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneNodeCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneNodeCollection[] = L"Windows.UI.Composition.Scenes.ISceneNodeCollection";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("29ada101-2dd9-4332-be63-60d2cf4269f2")
                    ISceneNodeCollection : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneNodeCollection = __uuidof(ISceneNodeCollection);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneNodeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneNodeStatics[] = L"Windows.UI.Composition.Scenes.ISceneNodeStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("579a0faa-be9d-4210-908c-93d15feed0b7")
                    ISceneNodeStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Composition::ICompositor* compositor,
                            ABI::Windows::UI::Composition::Scenes::ISceneNode** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneNodeStatics = __uuidof(ISceneNodeStatics);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneObject
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneObject
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneObject[] = L"Windows.UI.Composition.Scenes.ISceneObject";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("1e94249b-0f1b-49eb-a819-877d8450005b")
                    ISceneObject : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneObject = __uuidof(ISceneObject);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneObjectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneObject
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneObjectFactory[] = L"Windows.UI.Composition.Scenes.ISceneObjectFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("14fe799a-33e4-52ef-956c-44229d21f2c1")
                    ISceneObjectFactory : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneObjectFactory = __uuidof(ISceneObjectFactory);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.IScenePbrMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.ScenePbrMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_IScenePbrMaterial[] = L"Windows.UI.Composition.Scenes.IScenePbrMaterial";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("aab6ebbe-d680-46df-8294-b6800a9f95e7")
                    IScenePbrMaterial : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AlphaCutoff(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AlphaCutoff(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AlphaMode(
                            ABI::Windows::UI::Composition::Scenes::SceneAlphaMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AlphaMode(
                            ABI::Windows::UI::Composition::Scenes::SceneAlphaMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmissiveInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_EmissiveInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmissiveFactor(
                            ABI::Windows::Foundation::Numerics::Vector3* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_EmissiveFactor(
                            ABI::Windows::Foundation::Numerics::Vector3 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDoubleSided(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsDoubleSided(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NormalInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NormalInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NormalScale(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NormalScale(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OcclusionInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OcclusionInput(
                            ABI::Windows::UI::Composition::Scenes::ISceneMaterialInput* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OcclusionStrength(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OcclusionStrength(
                            FLOAT value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IScenePbrMaterial = __uuidof(IScenePbrMaterial);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.IScenePbrMaterialFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.ScenePbrMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_IScenePbrMaterialFactory[] = L"Windows.UI.Composition.Scenes.IScenePbrMaterialFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("2e3f3dfe-0b85-5727-b5be-b7d3cbac37fa")
                    IScenePbrMaterialFactory : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IScenePbrMaterialFactory = __uuidof(IScenePbrMaterialFactory);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneRendererComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneRendererComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneRendererComponent[] = L"Windows.UI.Composition.Scenes.ISceneRendererComponent";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("f1acb857-cf4f-4025-9b25-a2d1944cf507")
                    ISceneRendererComponent : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneRendererComponent = __uuidof(ISceneRendererComponent);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneRendererComponentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneRendererComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneRendererComponentFactory[] = L"Windows.UI.Composition.Scenes.ISceneRendererComponentFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("1db6ed6c-aa2c-5967-9035-56352dc69658")
                    ISceneRendererComponentFactory : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISceneRendererComponentFactory = __uuidof(ISceneRendererComponentFactory);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneSurfaceMaterialInput[] = L"Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInput";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("9937da5c-a9ca-4cfc-b3aa-088356518742")
                    ISceneSurfaceMaterialInput : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BitmapInterpolationMode(
                            ABI::Windows::UI::Composition::CompositionBitmapInterpolationMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_BitmapInterpolationMode(
                            ABI::Windows::UI::Composition::CompositionBitmapInterpolationMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Surface(
                            ABI::Windows::UI::Composition::ICompositionSurface** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Surface(
                            ABI::Windows::UI::Composition::ICompositionSurface* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WrappingUMode(
                            ABI::Windows::UI::Composition::Scenes::SceneWrappingMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_WrappingUMode(
                            ABI::Windows::UI::Composition::Scenes::SceneWrappingMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WrappingVMode(
                            ABI::Windows::UI::Composition::Scenes::SceneWrappingMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_WrappingVMode(
                            ABI::Windows::UI::Composition::Scenes::SceneWrappingMode value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneSurfaceMaterialInput = __uuidof(ISceneSurfaceMaterialInput);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInputStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneSurfaceMaterialInputStatics[] = L"Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInputStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("5a2394d3-6429-4589-bbcf-b84f4f3cfbfe")
                    ISceneSurfaceMaterialInputStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Composition::ICompositor* compositor,
                            ABI::Windows::UI::Composition::Scenes::ISceneSurfaceMaterialInput** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneSurfaceMaterialInputStatics = __uuidof(ISceneSurfaceMaterialInputStatics);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneVisual
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneVisual
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneVisual[] = L"Windows.UI.Composition.Scenes.ISceneVisual";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("8e672c1e-d734-47b1-be14-3d694ffa4301")
                    ISceneVisual : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Root(
                            ABI::Windows::UI::Composition::Scenes::ISceneNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Root(
                            ABI::Windows::UI::Composition::Scenes::ISceneNode* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneVisual = __uuidof(ISceneVisual);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneVisualStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneVisual
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneVisualStatics[] = L"Windows.UI.Composition.Scenes.ISceneVisualStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Scenes {
                    MIDL_INTERFACE("b8347e9a-50aa-4527-8d34-de4cb8ea88b4")
                    ISceneVisualStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Composition::ICompositor* compositor,
                            ABI::Windows::UI::Composition::Scenes::ISceneVisual** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneVisualStatics = __uuidof(ISceneVisualStatics);
                } /* Scenes */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneBoundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneBoundingBox ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneBoundingBox_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneBoundingBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneBoundingBox[] = L"Windows.UI.Composition.Scenes.SceneBoundingBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneComponent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneComponent_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneComponent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneComponent[] = L"Windows.UI.Composition.Scenes.SceneComponent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneComponentCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneComponentCollection
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Composition.Scenes.SceneComponent> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Composition.Scenes.SceneComponent>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneComponentCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneComponentCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneComponentCollection[] = L"Windows.UI.Composition.Scenes.SceneComponentCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMaterial_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMaterial[] = L"Windows.UI.Composition.Scenes.SceneMaterial";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMaterialInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMaterialInput ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMaterialInput_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMaterialInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMaterialInput[] = L"Windows.UI.Composition.Scenes.SceneMaterialInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMesh
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneMeshStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMesh ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMesh_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMesh_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMesh[] = L"Windows.UI.Composition.Scenes.SceneMesh";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMeshMaterialAttributeMap ** Default Interface **
 *    Windows.Foundation.Collections.IMap`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMeshMaterialAttributeMap_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMeshMaterialAttributeMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMeshMaterialAttributeMap[] = L"Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMeshRendererComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneMeshRendererComponentStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMeshRendererComponent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMeshRendererComponent_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMeshRendererComponent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMeshRendererComponent[] = L"Windows.UI.Composition.Scenes.SceneMeshRendererComponent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterialStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMetallicRoughnessMaterial_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMetallicRoughnessMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMetallicRoughnessMaterial[] = L"Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneModelTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneModelTransform ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneModelTransform_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneModelTransform_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneModelTransform[] = L"Windows.UI.Composition.Scenes.SceneModelTransform";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneNodeStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneNode ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneNode_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneNode[] = L"Windows.UI.Composition.Scenes.SceneNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneNodeCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneNodeCollection
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Composition.Scenes.SceneNode> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Composition.Scenes.SceneNode>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneNodeCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneNodeCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneNodeCollection[] = L"Windows.UI.Composition.Scenes.SceneNodeCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneObject
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneObject ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneObject_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneObject_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneObject[] = L"Windows.UI.Composition.Scenes.SceneObject";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.ScenePbrMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.IScenePbrMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_ScenePbrMaterial_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_ScenePbrMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_ScenePbrMaterial[] = L"Windows.UI.Composition.Scenes.ScenePbrMaterial";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneRendererComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneRendererComponent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneRendererComponent_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneRendererComponent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneRendererComponent[] = L"Windows.UI.Composition.Scenes.SceneRendererComponent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInputStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInput ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneSurfaceMaterialInput_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneSurfaceMaterialInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneSurfaceMaterialInput[] = L"Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneVisual
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneVisualStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneVisual ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneVisual_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneVisual_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneVisual[] = L"Windows.UI.Composition.Scenes.SceneVisual";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics;

#endif // ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAttributeSemantic __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAttributeSemantic;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAttributeSemantic* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent;

typedef struct __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl;

interface __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent;

typedef struct __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneComponent** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl;

interface __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode;

typedef struct __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl;

interface __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode;

typedef struct __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        __FIIterator_1_Windows__CUI__CComposition__CScenes__CSceneNode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl;

interface __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

typedef struct __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING key,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAttributeSemantic* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic** first,
        __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl;

interface __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic;

typedef struct __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING key,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAttributeSemantic* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        __FIMapView_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING key,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAttributeSemantic value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic* This);

    END_INTERFACE
} __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl;

interface __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic
{
    CONST_VTBL struct __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemanticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_Windows__CUI__CComposition__CScenes__CSceneAttributeSemantic_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent;

typedef struct __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl;

interface __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode;

typedef struct __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl;

interface __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent;

typedef struct __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneComponent** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl;

interface __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode;

typedef struct __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        __FIVectorView_1_Windows__CUI__CComposition__CScenes__CSceneNode** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl;

interface __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBuffer __x_ABI_CWindows_CFoundation_CIMemoryBuffer;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector4 __x_ABI_CWindows_CFoundation_CNumerics_CVector4;

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat;

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPrimitiveTopology __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPrimitiveTopology;

typedef enum __x_ABI_CWindows_CUI_CComposition_CCompositionBitmapInterpolationMode __x_ABI_CWindows_CUI_CComposition_CCompositionBitmapInterpolationMode;

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CICompositor __x_ABI_CWindows_CUI_CComposition_CICompositor;

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositionSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositionSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CICompositionSurface __x_ABI_CWindows_CUI_CComposition_CICompositionSurface;

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositionSurface_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAlphaMode __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAlphaMode;

typedef enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneComponentType __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneComponentType;

typedef enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneWrappingMode __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneWrappingMode;

/*
 *
 * Struct Windows.UI.Composition.Scenes.SceneAlphaMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAlphaMode
{
    SceneAlphaMode_Opaque = 0,
    SceneAlphaMode_AlphaTest = 1,
    SceneAlphaMode_Blend = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Composition.Scenes.SceneAttributeSemantic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAttributeSemantic
{
    SceneAttributeSemantic_Index = 0,
    SceneAttributeSemantic_Vertex = 1,
    SceneAttributeSemantic_Normal = 2,
    SceneAttributeSemantic_TexCoord0 = 3,
    SceneAttributeSemantic_TexCoord1 = 4,
    SceneAttributeSemantic_Color = 5,
    SceneAttributeSemantic_Tangent = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Composition.Scenes.SceneComponentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneComponentType
{
    SceneComponentType_MeshRendererComponent = 0,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.Composition.Scenes.SceneWrappingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneWrappingMode
{
    SceneWrappingMode_ClampToEdge = 0,
    SceneWrappingMode_MirroredRepeat = 1,
    SceneWrappingMode_Repeat = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneBoundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneBoundingBox
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneBoundingBox[] = L"Windows.UI.Composition.Scenes.ISceneBoundingBox";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Center)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_Extents)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_Max)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_Min)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBoxVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_get_Center(This, value) \
    ((This)->lpVtbl->get_Center(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_get_Extents(This, value) \
    ((This)->lpVtbl->get_Extents(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_get_Max(This, value) \
    ((This)->lpVtbl->get_Max(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_get_Min(This, value) \
    ((This)->lpVtbl->get_Min(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneComponent[] = L"Windows.UI.Composition.Scenes.ISceneComponent";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ComponentType)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneComponentType* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_get_ComponentType(This, value) \
    ((This)->lpVtbl->get_ComponentType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneComponentCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneComponentCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneComponentCollection[] = L"Windows.UI.Composition.Scenes.ISceneComponentCollection";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollectionVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneComponentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneComponentFactory[] = L"Windows.UI.Composition.Scenes.ISceneComponentFactory";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactoryVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMaterial[] = L"Windows.UI.Composition.Scenes.ISceneMaterial";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMaterialFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMaterialFactory[] = L"Windows.UI.Composition.Scenes.ISceneMaterialFactory";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactoryVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMaterialInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMaterialInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMaterialInput[] = L"Windows.UI.Composition.Scenes.ISceneMaterialInput";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMaterialInputFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMaterialInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMaterialInputFactory[] = L"Windows.UI.Composition.Scenes.ISceneMaterialInputFactory";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactoryVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInputFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMesh
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMesh
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMesh[] = L"Windows.UI.Composition.Scenes.ISceneMesh";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Bounds)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneBoundingBox** value);
    HRESULT (STDMETHODCALLTYPE* get_PrimitiveTopology)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPrimitiveTopology* value);
    HRESULT (STDMETHODCALLTYPE* put_PrimitiveTopology)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPrimitiveTopology value);
    HRESULT (STDMETHODCALLTYPE* FillMeshAttribute)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAttributeSemantic semantic,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat format,
        __x_ABI_CWindows_CFoundation_CIMemoryBuffer* memory);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_get_Bounds(This, value) \
    ((This)->lpVtbl->get_Bounds(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_get_PrimitiveTopology(This, value) \
    ((This)->lpVtbl->get_PrimitiveTopology(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_put_PrimitiveTopology(This, value) \
    ((This)->lpVtbl->put_PrimitiveTopology(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_FillMeshAttribute(This, semantic, format, memory) \
    ((This)->lpVtbl->FillMeshAttribute(This, semantic, format, memory))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMeshMaterialAttributeMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMeshMaterialAttributeMap[] = L"Windows.UI.Composition.Scenes.ISceneMeshMaterialAttributeMap";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMapVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMeshRendererComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMeshRendererComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMeshRendererComponent[] = L"Windows.UI.Composition.Scenes.ISceneMeshRendererComponent";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Material)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial** value);
    HRESULT (STDMETHODCALLTYPE* put_Material)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterial* value);
    HRESULT (STDMETHODCALLTYPE* get_Mesh)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh** value);
    HRESULT (STDMETHODCALLTYPE* put_Mesh)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh* value);
    HRESULT (STDMETHODCALLTYPE* get_UVMappings)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshMaterialAttributeMap** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_get_Material(This, value) \
    ((This)->lpVtbl->get_Material(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_put_Material(This, value) \
    ((This)->lpVtbl->put_Material(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_get_Mesh(This, value) \
    ((This)->lpVtbl->get_Mesh(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_put_Mesh(This, value) \
    ((This)->lpVtbl->put_Mesh(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_get_UVMappings(This, value) \
    ((This)->lpVtbl->get_UVMappings(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMeshRendererComponentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMeshRendererComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMeshRendererComponentStatics[] = L"Windows.UI.Composition.Scenes.ISceneMeshRendererComponentStatics";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositor* compositor,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponent** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStaticsVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_Create(This, compositor, result) \
    ((This)->lpVtbl->Create(This, compositor, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshRendererComponentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMeshStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMesh
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMeshStatics[] = L"Windows.UI.Composition.Scenes.ISceneMeshStatics";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositor* compositor,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMesh** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStaticsVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_Create(This, compositor, result) \
    ((This)->lpVtbl->Create(This, compositor, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMeshStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMetallicRoughnessMaterial[] = L"Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterial";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BaseColorInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput** value);
    HRESULT (STDMETHODCALLTYPE* put_BaseColorInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* value);
    HRESULT (STDMETHODCALLTYPE* get_BaseColorFactor)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector4* value);
    HRESULT (STDMETHODCALLTYPE* put_BaseColorFactor)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector4 value);
    HRESULT (STDMETHODCALLTYPE* get_MetallicFactor)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_MetallicFactor)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_MetallicRoughnessInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput** value);
    HRESULT (STDMETHODCALLTYPE* put_MetallicRoughnessInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* value);
    HRESULT (STDMETHODCALLTYPE* get_RoughnessFactor)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_RoughnessFactor)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_get_BaseColorInput(This, value) \
    ((This)->lpVtbl->get_BaseColorInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_put_BaseColorInput(This, value) \
    ((This)->lpVtbl->put_BaseColorInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_get_BaseColorFactor(This, value) \
    ((This)->lpVtbl->get_BaseColorFactor(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_put_BaseColorFactor(This, value) \
    ((This)->lpVtbl->put_BaseColorFactor(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_get_MetallicFactor(This, value) \
    ((This)->lpVtbl->get_MetallicFactor(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_put_MetallicFactor(This, value) \
    ((This)->lpVtbl->put_MetallicFactor(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_get_MetallicRoughnessInput(This, value) \
    ((This)->lpVtbl->get_MetallicRoughnessInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_put_MetallicRoughnessInput(This, value) \
    ((This)->lpVtbl->put_MetallicRoughnessInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_get_RoughnessFactor(This, value) \
    ((This)->lpVtbl->get_RoughnessFactor(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_put_RoughnessFactor(This, value) \
    ((This)->lpVtbl->put_RoughnessFactor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterialStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneMetallicRoughnessMaterialStatics[] = L"Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterialStatics";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositor* compositor,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterial** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStaticsVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_Create(This, compositor, result) \
    ((This)->lpVtbl->Create(This, compositor, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMetallicRoughnessMaterialStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneModelTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneModelTransform
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneModelTransform[] = L"Windows.UI.Composition.Scenes.ISceneModelTransform";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransformVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);
    HRESULT (STDMETHODCALLTYPE* put_Orientation)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion value);
    HRESULT (STDMETHODCALLTYPE* get_RotationAngle)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_RotationAngle)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_RotationAngleInDegrees)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_RotationAngleInDegrees)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_RotationAxis)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_RotationAxis)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);
    HRESULT (STDMETHODCALLTYPE* get_Scale)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_Scale)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);
    HRESULT (STDMETHODCALLTYPE* get_Translation)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_Translation)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransformVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransformVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_put_Orientation(This, value) \
    ((This)->lpVtbl->put_Orientation(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_get_RotationAngle(This, value) \
    ((This)->lpVtbl->get_RotationAngle(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_put_RotationAngle(This, value) \
    ((This)->lpVtbl->put_RotationAngle(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_get_RotationAngleInDegrees(This, value) \
    ((This)->lpVtbl->get_RotationAngleInDegrees(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_put_RotationAngleInDegrees(This, value) \
    ((This)->lpVtbl->put_RotationAngleInDegrees(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_get_RotationAxis(This, value) \
    ((This)->lpVtbl->get_RotationAxis(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_put_RotationAxis(This, value) \
    ((This)->lpVtbl->put_RotationAxis(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_get_Scale(This, value) \
    ((This)->lpVtbl->get_Scale(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_put_Scale(This, value) \
    ((This)->lpVtbl->put_Scale(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_get_Translation(This, value) \
    ((This)->lpVtbl->get_Translation(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_put_Translation(This, value) \
    ((This)->lpVtbl->put_Translation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneNode[] = L"Windows.UI.Composition.Scenes.ISceneNode";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Children)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneNode** value);
    HRESULT (STDMETHODCALLTYPE* get_Components)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        __FIVector_1_Windows__CUI__CComposition__CScenes__CSceneComponent** value);
    HRESULT (STDMETHODCALLTYPE* get_Parent)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** value);
    HRESULT (STDMETHODCALLTYPE* get_Transform)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneModelTransform** value);
    HRESULT (STDMETHODCALLTYPE* FindFirstComponentOfType)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneComponentType value,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneComponent** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_get_Children(This, value) \
    ((This)->lpVtbl->get_Children(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_get_Components(This, value) \
    ((This)->lpVtbl->get_Components(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_get_Parent(This, value) \
    ((This)->lpVtbl->get_Parent(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_get_Transform(This, value) \
    ((This)->lpVtbl->get_Transform(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_FindFirstComponentOfType(This, value, result) \
    ((This)->lpVtbl->FindFirstComponentOfType(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneNodeCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneNodeCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneNodeCollection[] = L"Windows.UI.Composition.Scenes.ISceneNodeCollection";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollectionVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneNodeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneNodeStatics[] = L"Windows.UI.Composition.Scenes.ISceneNodeStatics";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositor* compositor,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStaticsVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_Create(This, compositor, result) \
    ((This)->lpVtbl->Create(This, compositor, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNodeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneObject
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneObject
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneObject[] = L"Windows.UI.Composition.Scenes.ISceneObject";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneObjectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneObject
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneObjectFactory[] = L"Windows.UI.Composition.Scenes.ISceneObjectFactory";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactoryVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneObjectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.IScenePbrMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.ScenePbrMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_IScenePbrMaterial[] = L"Windows.UI.Composition.Scenes.IScenePbrMaterial";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlphaCutoff)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_AlphaCutoff)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_AlphaMode)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAlphaMode* value);
    HRESULT (STDMETHODCALLTYPE* put_AlphaMode)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneAlphaMode value);
    HRESULT (STDMETHODCALLTYPE* get_EmissiveInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput** value);
    HRESULT (STDMETHODCALLTYPE* put_EmissiveInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* value);
    HRESULT (STDMETHODCALLTYPE* get_EmissiveFactor)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_EmissiveFactor)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);
    HRESULT (STDMETHODCALLTYPE* get_IsDoubleSided)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsDoubleSided)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_NormalInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput** value);
    HRESULT (STDMETHODCALLTYPE* put_NormalInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* value);
    HRESULT (STDMETHODCALLTYPE* get_NormalScale)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_NormalScale)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_OcclusionInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput** value);
    HRESULT (STDMETHODCALLTYPE* put_OcclusionInput)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneMaterialInput* value);
    HRESULT (STDMETHODCALLTYPE* get_OcclusionStrength)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_OcclusionStrength)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_AlphaCutoff(This, value) \
    ((This)->lpVtbl->get_AlphaCutoff(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_AlphaCutoff(This, value) \
    ((This)->lpVtbl->put_AlphaCutoff(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_AlphaMode(This, value) \
    ((This)->lpVtbl->get_AlphaMode(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_AlphaMode(This, value) \
    ((This)->lpVtbl->put_AlphaMode(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_EmissiveInput(This, value) \
    ((This)->lpVtbl->get_EmissiveInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_EmissiveInput(This, value) \
    ((This)->lpVtbl->put_EmissiveInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_EmissiveFactor(This, value) \
    ((This)->lpVtbl->get_EmissiveFactor(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_EmissiveFactor(This, value) \
    ((This)->lpVtbl->put_EmissiveFactor(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_IsDoubleSided(This, value) \
    ((This)->lpVtbl->get_IsDoubleSided(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_IsDoubleSided(This, value) \
    ((This)->lpVtbl->put_IsDoubleSided(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_NormalInput(This, value) \
    ((This)->lpVtbl->get_NormalInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_NormalInput(This, value) \
    ((This)->lpVtbl->put_NormalInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_NormalScale(This, value) \
    ((This)->lpVtbl->get_NormalScale(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_NormalScale(This, value) \
    ((This)->lpVtbl->put_NormalScale(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_OcclusionInput(This, value) \
    ((This)->lpVtbl->get_OcclusionInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_OcclusionInput(This, value) \
    ((This)->lpVtbl->put_OcclusionInput(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_get_OcclusionStrength(This, value) \
    ((This)->lpVtbl->get_OcclusionStrength(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_put_OcclusionStrength(This, value) \
    ((This)->lpVtbl->put_OcclusionStrength(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterial_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.IScenePbrMaterialFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.ScenePbrMaterial
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_IScenePbrMaterialFactory[] = L"Windows.UI.Composition.Scenes.IScenePbrMaterialFactory";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactoryVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CIScenePbrMaterialFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneRendererComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneRendererComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneRendererComponent[] = L"Windows.UI.Composition.Scenes.ISceneRendererComponent";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneRendererComponentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneRendererComponent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneRendererComponentFactory[] = L"Windows.UI.Composition.Scenes.ISceneRendererComponentFactory";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactoryVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneRendererComponentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneSurfaceMaterialInput[] = L"Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInput";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BitmapInterpolationMode)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        enum __x_ABI_CWindows_CUI_CComposition_CCompositionBitmapInterpolationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_BitmapInterpolationMode)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        enum __x_ABI_CWindows_CUI_CComposition_CCompositionBitmapInterpolationMode value);
    HRESULT (STDMETHODCALLTYPE* get_Surface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositionSurface** value);
    HRESULT (STDMETHODCALLTYPE* put_Surface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositionSurface* value);
    HRESULT (STDMETHODCALLTYPE* get_WrappingUMode)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneWrappingMode* value);
    HRESULT (STDMETHODCALLTYPE* put_WrappingUMode)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneWrappingMode value);
    HRESULT (STDMETHODCALLTYPE* get_WrappingVMode)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneWrappingMode* value);
    HRESULT (STDMETHODCALLTYPE* put_WrappingVMode)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput* This,
        enum __x_ABI_CWindows_CUI_CComposition_CScenes_CSceneWrappingMode value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_get_BitmapInterpolationMode(This, value) \
    ((This)->lpVtbl->get_BitmapInterpolationMode(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_put_BitmapInterpolationMode(This, value) \
    ((This)->lpVtbl->put_BitmapInterpolationMode(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_get_Surface(This, value) \
    ((This)->lpVtbl->get_Surface(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_put_Surface(This, value) \
    ((This)->lpVtbl->put_Surface(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_get_WrappingUMode(This, value) \
    ((This)->lpVtbl->get_WrappingUMode(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_put_WrappingUMode(This, value) \
    ((This)->lpVtbl->put_WrappingUMode(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_get_WrappingVMode(This, value) \
    ((This)->lpVtbl->get_WrappingVMode(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_put_WrappingVMode(This, value) \
    ((This)->lpVtbl->put_WrappingVMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInputStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneSurfaceMaterialInputStatics[] = L"Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInputStatics";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositor* compositor,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInput** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStaticsVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_Create(This, compositor, result) \
    ((This)->lpVtbl->Create(This, compositor, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneSurfaceMaterialInputStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneVisual
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneVisual
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneVisual[] = L"Windows.UI.Composition.Scenes.ISceneVisual";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Root)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode** value);
    HRESULT (STDMETHODCALLTYPE* put_Root)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual* This,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneNode* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_get_Root(This, value) \
    ((This)->lpVtbl->get_Root(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_put_Root(This, value) \
    ((This)->lpVtbl->put_Root(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Composition.Scenes.ISceneVisualStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Scenes.SceneVisual
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Scenes_ISceneVisualStatics[] = L"Windows.UI.Composition.Scenes.ISceneVisualStatics";
typedef struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositor* compositor,
        __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisual** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStaticsVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_Create(This, compositor, result) \
    ((This)->lpVtbl->Create(This, compositor, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CScenes_CISceneVisualStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneBoundingBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneBoundingBox ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneBoundingBox_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneBoundingBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneBoundingBox[] = L"Windows.UI.Composition.Scenes.SceneBoundingBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneComponent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneComponent_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneComponent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneComponent[] = L"Windows.UI.Composition.Scenes.SceneComponent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneComponentCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneComponentCollection
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Composition.Scenes.SceneComponent> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Composition.Scenes.SceneComponent>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneComponentCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneComponentCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneComponentCollection[] = L"Windows.UI.Composition.Scenes.SceneComponentCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMaterial_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMaterial[] = L"Windows.UI.Composition.Scenes.SceneMaterial";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMaterialInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMaterialInput ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMaterialInput_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMaterialInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMaterialInput[] = L"Windows.UI.Composition.Scenes.SceneMaterialInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMesh
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneMeshStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMesh ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMesh_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMesh_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMesh[] = L"Windows.UI.Composition.Scenes.SceneMesh";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMeshMaterialAttributeMap ** Default Interface **
 *    Windows.Foundation.Collections.IMap`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.UI.Composition.Scenes.SceneAttributeSemantic>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMeshMaterialAttributeMap_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMeshMaterialAttributeMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMeshMaterialAttributeMap[] = L"Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMeshRendererComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneMeshRendererComponentStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMeshRendererComponent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMeshRendererComponent_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMeshRendererComponent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMeshRendererComponent[] = L"Windows.UI.Composition.Scenes.SceneMeshRendererComponent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterialStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneMetallicRoughnessMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMetallicRoughnessMaterial_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneMetallicRoughnessMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneMetallicRoughnessMaterial[] = L"Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneModelTransform
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneModelTransform ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneModelTransform_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneModelTransform_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneModelTransform[] = L"Windows.UI.Composition.Scenes.SceneModelTransform";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneNodeStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneNode ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneNode_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneNode[] = L"Windows.UI.Composition.Scenes.SceneNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneNodeCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneNodeCollection
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Composition.Scenes.SceneNode> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Composition.Scenes.SceneNode>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneNodeCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneNodeCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneNodeCollection[] = L"Windows.UI.Composition.Scenes.SceneNodeCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneObject
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneObject ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneObject_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneObject_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneObject[] = L"Windows.UI.Composition.Scenes.SceneObject";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.ScenePbrMaterial
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.IScenePbrMaterial ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_ScenePbrMaterial_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_ScenePbrMaterial_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_ScenePbrMaterial[] = L"Windows.UI.Composition.Scenes.ScenePbrMaterial";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneRendererComponent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneRendererComponent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneRendererComponent_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneRendererComponent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneRendererComponent[] = L"Windows.UI.Composition.Scenes.SceneRendererComponent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInputStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneSurfaceMaterialInput ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneSurfaceMaterialInput_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneSurfaceMaterialInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneSurfaceMaterialInput[] = L"Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Composition.Scenes.SceneVisual
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Composition.Scenes.ISceneVisualStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Scenes.ISceneVisual ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneVisual_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Scenes_SceneVisual_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Scenes_SceneVisual[] = L"Windows.UI.Composition.Scenes.SceneVisual";
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
#endif // __windows2Eui2Ecomposition2Escenes_p_h__

#endif // __windows2Eui2Ecomposition2Escenes_h__
