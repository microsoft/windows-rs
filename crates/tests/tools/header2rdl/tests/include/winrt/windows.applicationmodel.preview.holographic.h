
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
#ifndef __windows2Eapplicationmodel2Epreview2Eholographic_h__
#define __windows2Eapplicationmodel2Epreview2Eholographic_h__
#ifndef __windows2Eapplicationmodel2Epreview2Eholographic_p_h__
#define __windows2Eapplicationmodel2Epreview2Eholographic_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Activation.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.Perception.Spatial.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Holographic {
                    interface IHolographicApplicationPreviewStatics;
                } /* Holographic */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics ABI::Windows::ApplicationModel::Preview::Holographic::IHolographicApplicationPreviewStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Holographic {
                    interface IHolographicKeyboardPlacementOverridePreview;
                } /* Holographic */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview ABI::Windows::ApplicationModel::Preview::Holographic::IHolographicKeyboardPlacementOverridePreview

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Holographic {
                    interface IHolographicKeyboardPlacementOverridePreviewStatics;
                } /* Holographic */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics ABI::Windows::ApplicationModel::Preview::Holographic::IHolographicKeyboardPlacementOverridePreviewStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

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
        namespace ApplicationModel {
            namespace Preview {
                namespace Holographic {
                    class HolographicKeyboardPlacementOverridePreview;
                } /* Holographic */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Preview.Holographic.IHolographicApplicationPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Holographic_IHolographicApplicationPreviewStatics[] = L"Windows.ApplicationModel.Preview.Holographic.IHolographicApplicationPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Holographic {
                    MIDL_INTERFACE("fe038691-2a3a-45a9-a208-7bed691919f3")
                    IHolographicApplicationPreviewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsCurrentViewPresentedOnHolographicDisplay(
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsHolographicActivation(
                            ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs* activatedEventArgs,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHolographicApplicationPreviewStatics = __uuidof(IHolographicApplicationPreviewStatics);
                } /* Holographic */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Holographic_IHolographicKeyboardPlacementOverridePreview[] = L"Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreview";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Holographic {
                    MIDL_INTERFACE("c8a8ce3a-dfde-5a14-8d5f-182c526dd9c4")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                    IHolographicKeyboardPlacementOverridePreview : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        virtual HRESULT STDMETHODCALLTYPE SetPlacementOverride(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::Foundation::Numerics::Vector3 topCenterPosition,
                            ABI::Windows::Foundation::Numerics::Vector3 normal
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        virtual HRESULT STDMETHODCALLTYPE SetPlacementOverrideWithMaxSize(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::Foundation::Numerics::Vector3 topCenterPosition,
                            ABI::Windows::Foundation::Numerics::Vector3 normal,
                            ABI::Windows::Foundation::Numerics::Vector2 maxSize
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        virtual HRESULT STDMETHODCALLTYPE ResetPlacementOverride(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHolographicKeyboardPlacementOverridePreview = __uuidof(IHolographicKeyboardPlacementOverridePreview);
                } /* Holographic */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Holographic_IHolographicKeyboardPlacementOverridePreviewStatics[] = L"Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreviewStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Preview {
                namespace Holographic {
                    MIDL_INTERFACE("202e6039-1ff6-5a06-aac4-a5e24fa3ec4b")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                    IHolographicKeyboardPlacementOverridePreviewStatics : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::ApplicationModel::Preview::Holographic::IHolographicKeyboardPlacementOverridePreview** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHolographicKeyboardPlacementOverridePreviewStatics = __uuidof(IHolographicKeyboardPlacementOverridePreviewStatics);
                } /* Holographic */
            } /* Preview */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Preview.Holographic.IHolographicApplicationPreviewStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Holographic_HolographicApplicationPreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Holographic_HolographicApplicationPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Holographic_HolographicApplicationPreview[] = L"Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreviewStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Holographic_HolographicKeyboardPlacementOverridePreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Holographic_HolographicKeyboardPlacementOverridePreview_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Holographic_HolographicKeyboardPlacementOverridePreview[] = L"Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 __x_ABI_CWindows_CFoundation_CNumerics_CVector2;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.Preview.Holographic.IHolographicApplicationPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Holographic_IHolographicApplicationPreviewStatics[] = L"Windows.ApplicationModel.Preview.Holographic.IHolographicApplicationPreviewStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsCurrentViewPresentedOnHolographicDisplay)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsHolographicActivation)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics* This,
        __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* activatedEventArgs,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_IsCurrentViewPresentedOnHolographicDisplay(This, result) \
    ((This)->lpVtbl->IsCurrentViewPresentedOnHolographicDisplay(This, result))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_IsHolographicActivation(This, activatedEventArgs, result) \
    ((This)->lpVtbl->IsHolographicActivation(This, activatedEventArgs, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicApplicationPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Holographic_IHolographicKeyboardPlacementOverridePreview[] = L"Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreview";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    HRESULT (STDMETHODCALLTYPE* SetPlacementOverride)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 topCenterPosition,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 normal);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    HRESULT (STDMETHODCALLTYPE* SetPlacementOverrideWithMaxSize)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 topCenterPosition,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 normal,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 maxSize);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    HRESULT (STDMETHODCALLTYPE* ResetPlacementOverride)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_SetPlacementOverride(This, coordinateSystem, topCenterPosition, normal) \
    ((This)->lpVtbl->SetPlacementOverride(This, coordinateSystem, topCenterPosition, normal))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_SetPlacementOverrideWithMaxSize(This, coordinateSystem, topCenterPosition, normal, maxSize) \
    ((This)->lpVtbl->SetPlacementOverrideWithMaxSize(This, coordinateSystem, topCenterPosition, normal, maxSize))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_ResetPlacementOverride(This) \
    ((This)->lpVtbl->ResetPlacementOverride(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Preview_Holographic_IHolographicKeyboardPlacementOverridePreviewStatics[] = L"Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreviewStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics* This,
        __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreview** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#define __x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPreview_CHolographic_CIHolographicKeyboardPlacementOverridePreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Preview.Holographic.IHolographicApplicationPreviewStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Holographic_HolographicApplicationPreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Holographic_HolographicApplicationPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Holographic_HolographicApplicationPreview[] = L"Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreviewStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Preview_Holographic_HolographicKeyboardPlacementOverridePreview_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Preview_Holographic_HolographicKeyboardPlacementOverridePreview_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
DEPRECATED("Use Windows.ApplicationModel.Holographic.HolographicKeyboard instead of Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Preview_Holographic_HolographicKeyboardPlacementOverridePreview[] = L"Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview";
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
#endif // __windows2Eapplicationmodel2Epreview2Eholographic_p_h__

#endif // __windows2Eapplicationmodel2Epreview2Eholographic_h__
