
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
#ifndef __windows2Eapplicationmodel2Eholographic_h__
#define __windows2Eapplicationmodel2Eholographic_h__
#ifndef __windows2Eapplicationmodel2Eholographic_p_h__
#define __windows2Eapplicationmodel2Eholographic_p_h__


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
#include "Windows.Perception.Spatial.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Holographic {
                interface IHolographicKeyboard;
            } /* Holographic */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard ABI::Windows::ApplicationModel::Holographic::IHolographicKeyboard

#endif // ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Holographic {
                interface IHolographicKeyboardStatics;
            } /* Holographic */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics ABI::Windows::ApplicationModel::Holographic::IHolographicKeyboardStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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
            namespace Holographic {
                class HolographicKeyboard;
            } /* Holographic */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Holographic.IHolographicKeyboard
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Holographic.HolographicKeyboard
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Holographic_IHolographicKeyboard[] = L"Windows.ApplicationModel.Holographic.IHolographicKeyboard";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Holographic {
                MIDL_INTERFACE("07dd0893-aa21-5e6f-a91b-11b2b3fd7be3")
                IHolographicKeyboard : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetPlacementOverride(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Foundation::Numerics::Vector3 topCenterPosition,
                        ABI::Windows::Foundation::Numerics::Quaternion orientation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPlacementOverrideWithMaxSize(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Foundation::Numerics::Vector3 topCenterPosition,
                        ABI::Windows::Foundation::Numerics::Quaternion orientation,
                        ABI::Windows::Foundation::Numerics::Vector2 maxSize
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ResetPlacementOverride(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicKeyboard = __uuidof(IHolographicKeyboard);
            } /* Holographic */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.ApplicationModel.Holographic.IHolographicKeyboardStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Holographic.HolographicKeyboard
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Holographic_IHolographicKeyboardStatics[] = L"Windows.ApplicationModel.Holographic.IHolographicKeyboardStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Holographic {
                MIDL_INTERFACE("b676c624-63d7-58cf-b06b-08baa032a23f")
                IHolographicKeyboardStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::ApplicationModel::Holographic::IHolographicKeyboard** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHolographicKeyboardStatics = __uuidof(IHolographicKeyboardStatics);
            } /* Holographic */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Class Windows.ApplicationModel.Holographic.HolographicKeyboard
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Holographic.IHolographicKeyboardStatics interface starting with version 11.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Holographic.IHolographicKeyboard ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Holographic_HolographicKeyboard_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Holographic_HolographicKeyboard_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Holographic_HolographicKeyboard[] = L"Windows.ApplicationModel.Holographic.HolographicKeyboard";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard;

#endif // ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 __x_ABI_CWindows_CFoundation_CNumerics_CVector2;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.Holographic.IHolographicKeyboard
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Holographic.HolographicKeyboard
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Holographic_IHolographicKeyboard[] = L"Windows.ApplicationModel.Holographic.IHolographicKeyboard";
typedef struct __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetPlacementOverride)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 topCenterPosition,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion orientation);
    HRESULT (STDMETHODCALLTYPE* SetPlacementOverrideWithMaxSize)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 topCenterPosition,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion orientation,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector2 maxSize);
    HRESULT (STDMETHODCALLTYPE* ResetPlacementOverride)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardVtbl;

interface __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_SetPlacementOverride(This, coordinateSystem, topCenterPosition, orientation) \
    ((This)->lpVtbl->SetPlacementOverride(This, coordinateSystem, topCenterPosition, orientation))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_SetPlacementOverrideWithMaxSize(This, coordinateSystem, topCenterPosition, orientation, maxSize) \
    ((This)->lpVtbl->SetPlacementOverrideWithMaxSize(This, coordinateSystem, topCenterPosition, orientation, maxSize))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_ResetPlacementOverride(This) \
    ((This)->lpVtbl->ResetPlacementOverride(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.ApplicationModel.Holographic.IHolographicKeyboardStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Holographic.HolographicKeyboard
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Holographic_IHolographicKeyboardStatics[] = L"Windows.ApplicationModel.Holographic.IHolographicKeyboardStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics* This,
        __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboard** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CHolographic_CIHolographicKeyboardStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Class Windows.ApplicationModel.Holographic.HolographicKeyboard
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Holographic.IHolographicKeyboardStatics interface starting with version 11.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Holographic.IHolographicKeyboard ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Holographic_HolographicKeyboard_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Holographic_HolographicKeyboard_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Holographic_HolographicKeyboard[] = L"Windows.ApplicationModel.Holographic.HolographicKeyboard";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Eholographic_p_h__

#endif // __windows2Eapplicationmodel2Eholographic_h__
