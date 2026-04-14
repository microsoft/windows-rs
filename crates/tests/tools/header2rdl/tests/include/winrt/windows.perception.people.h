
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
#ifndef __windows2Eperception2Epeople_h__
#define __windows2Eperception2Epeople_h__
#ifndef __windows2Eperception2Epeople_p_h__
#define __windows2Eperception2Epeople_p_h__


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
#include "Windows.Perception.h"
#include "Windows.Perception.Spatial.h"
#include "Windows.UI.Input.h"
#include "Windows.UI.Input.Spatial.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IEyesPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose ABI::Windows::Perception::People::IEyesPose

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IEyesPoseStatics;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics ABI::Windows::Perception::People::IEyesPoseStatics

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IHandMeshObserver;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver ABI::Windows::Perception::People::IHandMeshObserver

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IHandMeshVertexState;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState ABI::Windows::Perception::People::IHandMeshVertexState

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IHandPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose ABI::Windows::Perception::People::IHandPose

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                interface IHeadPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose ABI::Windows::Perception::People::IHeadPose

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                typedef enum GazeInputAccessStatus : int GazeInputAccessStatus;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5764eb43-db4f-5fea-9bc5-af0158f26929"))
IAsyncOperation<enum ABI::Windows::UI::Input::GazeInputAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::UI::Input::GazeInputAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Input.GazeInputAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::UI::Input::GazeInputAccessStatus> __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("27a0f2c4-461f-50ab-af8f-d9d99f30b67d"))
IAsyncOperationCompletedHandler<enum ABI::Windows::UI::Input::GazeInputAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::UI::Input::GazeInputAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Input.GazeInputAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::UI::Input::GazeInputAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                typedef struct SpatialRay SpatialRay;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_USE
#define DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44dd686b-c7d8-582c-91c2-d98e604dcfd4"))
IReference<struct ABI::Windows::Perception::Spatial::SpatialRay> : IReference_impl<struct ABI::Windows::Perception::Spatial::SpatialRay>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Perception.Spatial.SpatialRay>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Perception::Spatial::SpatialRay> __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_t;
#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay ABI::Windows::Foundation::__FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

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
        namespace UI {
            namespace Input {
                namespace Spatial {
                    class SpatialInteractionSource;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Spatial {
                    interface ISpatialInteractionSource;
                } /* Spatial */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                typedef enum HandJointKind : int HandJointKind;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                typedef enum JointPoseAccuracy : int JointPoseAccuracy;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                typedef struct HandMeshVertex HandMeshVertex;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                typedef struct JointPose JointPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                class HandMeshVertexState;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                class HandPose;
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Perception.People.HandJointKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                enum HandJointKind : int
                {
                    HandJointKind_Palm = 0,
                    HandJointKind_Wrist = 1,
                    HandJointKind_ThumbMetacarpal = 2,
                    HandJointKind_ThumbProximal = 3,
                    HandJointKind_ThumbDistal = 4,
                    HandJointKind_ThumbTip = 5,
                    HandJointKind_IndexMetacarpal = 6,
                    HandJointKind_IndexProximal = 7,
                    HandJointKind_IndexIntermediate = 8,
                    HandJointKind_IndexDistal = 9,
                    HandJointKind_IndexTip = 10,
                    HandJointKind_MiddleMetacarpal = 11,
                    HandJointKind_MiddleProximal = 12,
                    HandJointKind_MiddleIntermediate = 13,
                    HandJointKind_MiddleDistal = 14,
                    HandJointKind_MiddleTip = 15,
                    HandJointKind_RingMetacarpal = 16,
                    HandJointKind_RingProximal = 17,
                    HandJointKind_RingIntermediate = 18,
                    HandJointKind_RingDistal = 19,
                    HandJointKind_RingTip = 20,
                    HandJointKind_LittleMetacarpal = 21,
                    HandJointKind_LittleProximal = 22,
                    HandJointKind_LittleIntermediate = 23,
                    HandJointKind_LittleDistal = 24,
                    HandJointKind_LittleTip = 25,
                };
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Perception.People.JointPoseAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                enum JointPoseAccuracy : int
                {
                    JointPoseAccuracy_High = 0,
                    JointPoseAccuracy_Approximate = 1,
                };
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Perception.People.HandMeshVertex
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                struct HandMeshVertex
                {
                    ABI::Windows::Foundation::Numerics::Vector3 Position;
                    ABI::Windows::Foundation::Numerics::Vector3 Normal;
                };
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Perception.People.JointPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                struct JointPose
                {
                    ABI::Windows::Foundation::Numerics::Quaternion Orientation;
                    ABI::Windows::Foundation::Numerics::Vector3 Position;
                    FLOAT Radius;
                    ABI::Windows::Perception::People::JointPoseAccuracy Accuracy;
                };
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IEyesPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.EyesPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IEyesPose[] = L"Windows.Perception.People.IEyesPose";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                MIDL_INTERFACE("682a9b23-8a1e-5b86-a060-906ffacb62a4")
                IEyesPose : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsCalibrationValid(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Gaze(
                        __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpdateTimestamp(
                        ABI::Windows::Perception::IPerceptionTimestamp** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEyesPose = __uuidof(IEyesPose);
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIEyesPose;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IEyesPoseStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.EyesPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IEyesPoseStatics[] = L"Windows.Perception.People.IEyesPoseStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                MIDL_INTERFACE("1cff7413-b21f-54c0-80c1-e60d994ca58c")
                IEyesPoseStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEyesPoseStatics = __uuidof(IEyesPoseStatics);
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IHandMeshObserver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.HandMeshObserver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IHandMeshObserver[] = L"Windows.Perception.People.IHandMeshObserver";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                MIDL_INTERFACE("85ae30cb-6fc3-55c4-a7b4-29e33896ca69")
                IHandMeshObserver : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::UI::Input::Spatial::ISpatialInteractionSource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TriangleIndexCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VertexCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTriangleIndices(
                        UINT32 indicesLength,
                        UINT16* indices
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVertexStateForPose(
                        ABI::Windows::Perception::People::IHandPose* handPose,
                        ABI::Windows::Perception::People::IHandMeshVertexState** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NeutralPose(
                        ABI::Windows::Perception::People::IHandPose** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NeutralPoseVersion(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModelId(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHandMeshObserver = __uuidof(IHandMeshObserver);
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IHandMeshVertexState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.HandMeshVertexState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IHandMeshVertexState[] = L"Windows.Perception.People.IHandMeshVertexState";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                MIDL_INTERFACE("046c5fef-1d8b-55de-ab2c-1cd424886d8f")
                IHandMeshVertexState : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CoordinateSystem(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVertices(
                        UINT32 verticesLength,
                        ABI::Windows::Perception::People::HandMeshVertex* vertices
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpdateTimestamp(
                        ABI::Windows::Perception::IPerceptionTimestamp** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHandMeshVertexState = __uuidof(IHandMeshVertexState);
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IHandPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.HandPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IHandPose[] = L"Windows.Perception.People.IHandPose";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                MIDL_INTERFACE("4d98e79a-bb08-5d09-91de-df0dd3fae46c")
                IHandPose : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryGetJoint(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        ABI::Windows::Perception::People::HandJointKind joint,
                        ABI::Windows::Perception::People::JointPose* jointPose,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetJoints(
                        ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                        UINT32 jointsLength,
                        ABI::Windows::Perception::People::HandJointKind* joints,
                        UINT32 jointPosesLength,
                        ABI::Windows::Perception::People::JointPose* jointPoses,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRelativeJoint(
                        ABI::Windows::Perception::People::HandJointKind joint,
                        ABI::Windows::Perception::People::HandJointKind referenceJoint,
                        ABI::Windows::Perception::People::JointPose* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRelativeJoints(
                        UINT32 jointsLength,
                        ABI::Windows::Perception::People::HandJointKind* joints,
                        UINT32 referenceJointsLength,
                        ABI::Windows::Perception::People::HandJointKind* referenceJoints,
                        UINT32 jointPosesLength,
                        ABI::Windows::Perception::People::JointPose* jointPoses
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHandPose = __uuidof(IHandPose);
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIHandPose;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IHeadPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.HeadPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IHeadPose[] = L"Windows.Perception.People.IHeadPose";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace People {
                MIDL_INTERFACE("7f5ac5a5-49db-379f-9429-32a2faf34fa6")
                IHeadPose : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ForwardDirection(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpDirection(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHeadPose = __uuidof(IHeadPose);
            } /* People */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIHeadPose;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.People.EyesPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.People.IEyesPoseStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IEyesPose ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Perception_People_EyesPose_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_EyesPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_EyesPose[] = L"Windows.Perception.People.EyesPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Perception.People.HandMeshObserver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IHandMeshObserver ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Perception_People_HandMeshObserver_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_HandMeshObserver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_HandMeshObserver[] = L"Windows.Perception.People.HandMeshObserver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Perception.People.HandMeshVertexState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IHandMeshVertexState ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Perception_People_HandMeshVertexState_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_HandMeshVertexState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_HandMeshVertexState[] = L"Windows.Perception.People.HandMeshVertexState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Perception.People.HandPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IHandPose ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Perception_People_HandPose_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_HandPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_HandPose[] = L"Windows.Perception.People.HandPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Perception.People.HeadPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IHeadPose ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_People_HeadPose_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_HeadPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_HeadPose[] = L"Windows.Perception.People.HeadPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIEyesPose __x_ABI_CWindows_CPerception_CPeople_CIEyesPose;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIHandPose __x_ABI_CWindows_CPerception_CPeople_CIHandPose;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CPeople_CIHeadPose __x_ABI_CWindows_CPerception_CPeople_CIHeadPose;

#endif // ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CUI_CInput_CGazeInputAccessStatus __x_ABI_CWindows_CUI_CInput_CGazeInputAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        enum __x_ABI_CWindows_CUI_CInput_CGazeInputAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus* This,
        __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CGazeInputAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialRay __x_ABI_CWindows_CPerception_CSpatial_CSpatialRay;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CPerception__CSpatial__CSpatialRay;

typedef struct __FIReference_1_Windows__CPerception__CSpatial__CSpatialRayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialRay* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialRay* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialRay* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialRay* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialRay* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialRay* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CPerception__CSpatial__CSpatialRay* This,
        struct __x_ABI_CWindows_CPerception_CSpatial_CSpatialRay* result);

    END_INTERFACE
} __FIReference_1_Windows__CPerception__CSpatial__CSpatialRayVtbl;

interface __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay
{
    CONST_VTBL struct __FIReference_1_Windows__CPerception__CSpatial__CSpatialRayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CPerception__CSpatial__CSpatialRay_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

#ifndef ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CIPerceptionTimestamp __x_ABI_CWindows_CPerception_CIPerceptionTimestamp;

#endif // ____x_ABI_CWindows_CPerception_CIPerceptionTimestamp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource;

#endif // ____x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CPerception_CPeople_CHandJointKind __x_ABI_CWindows_CPerception_CPeople_CHandJointKind;

typedef enum __x_ABI_CWindows_CPerception_CPeople_CJointPoseAccuracy __x_ABI_CWindows_CPerception_CPeople_CJointPoseAccuracy;

typedef struct __x_ABI_CWindows_CPerception_CPeople_CHandMeshVertex __x_ABI_CWindows_CPerception_CPeople_CHandMeshVertex;

typedef struct __x_ABI_CWindows_CPerception_CPeople_CJointPose __x_ABI_CWindows_CPerception_CPeople_CJointPose;

/*
 *
 * Struct Windows.Perception.People.HandJointKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CPerception_CPeople_CHandJointKind
{
    HandJointKind_Palm = 0,
    HandJointKind_Wrist = 1,
    HandJointKind_ThumbMetacarpal = 2,
    HandJointKind_ThumbProximal = 3,
    HandJointKind_ThumbDistal = 4,
    HandJointKind_ThumbTip = 5,
    HandJointKind_IndexMetacarpal = 6,
    HandJointKind_IndexProximal = 7,
    HandJointKind_IndexIntermediate = 8,
    HandJointKind_IndexDistal = 9,
    HandJointKind_IndexTip = 10,
    HandJointKind_MiddleMetacarpal = 11,
    HandJointKind_MiddleProximal = 12,
    HandJointKind_MiddleIntermediate = 13,
    HandJointKind_MiddleDistal = 14,
    HandJointKind_MiddleTip = 15,
    HandJointKind_RingMetacarpal = 16,
    HandJointKind_RingProximal = 17,
    HandJointKind_RingIntermediate = 18,
    HandJointKind_RingDistal = 19,
    HandJointKind_RingTip = 20,
    HandJointKind_LittleMetacarpal = 21,
    HandJointKind_LittleProximal = 22,
    HandJointKind_LittleIntermediate = 23,
    HandJointKind_LittleDistal = 24,
    HandJointKind_LittleTip = 25,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Perception.People.JointPoseAccuracy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CPerception_CPeople_CJointPoseAccuracy
{
    JointPoseAccuracy_High = 0,
    JointPoseAccuracy_Approximate = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Perception.People.HandMeshVertex
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
struct __x_ABI_CWindows_CPerception_CPeople_CHandMeshVertex
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Position;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Normal;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Perception.People.JointPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
struct __x_ABI_CWindows_CPerception_CPeople_CJointPose
{
    struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion Orientation;
    struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 Position;
    FLOAT Radius;
    enum __x_ABI_CWindows_CPerception_CPeople_CJointPoseAccuracy Accuracy;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IEyesPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.EyesPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IEyesPose[] = L"Windows.Perception.People.IEyesPose";
typedef struct __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsCalibrationValid)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Gaze)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This,
        __FIReference_1_Windows__CPerception__CSpatial__CSpatialRay** value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateTimestamp)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPose* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseVtbl;

interface __x_ABI_CWindows_CPerception_CPeople_CIEyesPose
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_get_IsCalibrationValid(This, value) \
    ((This)->lpVtbl->get_IsCalibrationValid(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_get_Gaze(This, value) \
    ((This)->lpVtbl->get_Gaze(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPose_get_UpdateTimestamp(This, value) \
    ((This)->lpVtbl->get_UpdateTimestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIEyesPose;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIEyesPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IEyesPoseStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.EyesPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IEyesPoseStatics[] = L"Windows.Perception.People.IEyesPoseStatics";
typedef struct __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics* This,
        __FIAsyncOperation_1_Windows__CUI__CInput__CGazeInputAccessStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_RequestAccessAsync(This, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIEyesPoseStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IHandMeshObserver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.HandMeshObserver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IHandMeshObserver[] = L"Windows.Perception.People.IHandMeshObserver";
typedef struct __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        __x_ABI_CWindows_CUI_CInput_CSpatial_CISpatialInteractionSource** value);
    HRESULT (STDMETHODCALLTYPE* get_TriangleIndexCount)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_VertexCount)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* GetTriangleIndices)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        UINT32 indicesLength,
        UINT16* indices);
    HRESULT (STDMETHODCALLTYPE* GetVertexStateForPose)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        __x_ABI_CWindows_CPerception_CPeople_CIHandPose* handPose,
        __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState** result);
    HRESULT (STDMETHODCALLTYPE* get_NeutralPose)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        __x_ABI_CWindows_CPerception_CPeople_CIHandPose** value);
    HRESULT (STDMETHODCALLTYPE* get_NeutralPoseVersion)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ModelId)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserverVtbl;

interface __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_get_TriangleIndexCount(This, value) \
    ((This)->lpVtbl->get_TriangleIndexCount(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_get_VertexCount(This, value) \
    ((This)->lpVtbl->get_VertexCount(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_GetTriangleIndices(This, indicesLength, indices) \
    ((This)->lpVtbl->GetTriangleIndices(This, indicesLength, indices))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_GetVertexStateForPose(This, handPose, result) \
    ((This)->lpVtbl->GetVertexStateForPose(This, handPose, result))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_get_NeutralPose(This, value) \
    ((This)->lpVtbl->get_NeutralPose(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_get_NeutralPoseVersion(This, value) \
    ((This)->lpVtbl->get_NeutralPoseVersion(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_get_ModelId(This, value) \
    ((This)->lpVtbl->get_ModelId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandMeshObserver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IHandMeshVertexState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.HandMeshVertexState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IHandMeshVertexState[] = L"Windows.Perception.People.IHandMeshVertexState";
typedef struct __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CoordinateSystem)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);
    HRESULT (STDMETHODCALLTYPE* GetVertices)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This,
        UINT32 verticesLength,
        struct __x_ABI_CWindows_CPerception_CPeople_CHandMeshVertex* vertices);
    HRESULT (STDMETHODCALLTYPE* get_UpdateTimestamp)(__x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState* This,
        __x_ABI_CWindows_CPerception_CIPerceptionTimestamp** value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexStateVtbl;

interface __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_get_CoordinateSystem(This, value) \
    ((This)->lpVtbl->get_CoordinateSystem(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_GetVertices(This, verticesLength, vertices) \
    ((This)->lpVtbl->GetVertices(This, verticesLength, vertices))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_get_UpdateTimestamp(This, value) \
    ((This)->lpVtbl->get_UpdateTimestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandMeshVertexState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IHandPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.HandPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIHandPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IHandPose[] = L"Windows.Perception.People.IHandPose";
typedef struct __x_ABI_CWindows_CPerception_CPeople_CIHandPoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetJoint)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        enum __x_ABI_CWindows_CPerception_CPeople_CHandJointKind joint,
        struct __x_ABI_CWindows_CPerception_CPeople_CJointPose* jointPose,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TryGetJoints)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        UINT32 jointsLength,
        enum __x_ABI_CWindows_CPerception_CPeople_CHandJointKind* joints,
        UINT32 jointPosesLength,
        struct __x_ABI_CWindows_CPerception_CPeople_CJointPose* jointPoses,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetRelativeJoint)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This,
        enum __x_ABI_CWindows_CPerception_CPeople_CHandJointKind joint,
        enum __x_ABI_CWindows_CPerception_CPeople_CHandJointKind referenceJoint,
        struct __x_ABI_CWindows_CPerception_CPeople_CJointPose* result);
    HRESULT (STDMETHODCALLTYPE* GetRelativeJoints)(__x_ABI_CWindows_CPerception_CPeople_CIHandPose* This,
        UINT32 jointsLength,
        enum __x_ABI_CWindows_CPerception_CPeople_CHandJointKind* joints,
        UINT32 referenceJointsLength,
        enum __x_ABI_CWindows_CPerception_CPeople_CHandJointKind* referenceJoints,
        UINT32 jointPosesLength,
        struct __x_ABI_CWindows_CPerception_CPeople_CJointPose* jointPoses);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CPeople_CIHandPoseVtbl;

interface __x_ABI_CWindows_CPerception_CPeople_CIHandPose
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CPeople_CIHandPoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_TryGetJoint(This, coordinateSystem, joint, jointPose, result) \
    ((This)->lpVtbl->TryGetJoint(This, coordinateSystem, joint, jointPose, result))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_TryGetJoints(This, coordinateSystem, jointsLength, joints, jointPosesLength, jointPoses, result) \
    ((This)->lpVtbl->TryGetJoints(This, coordinateSystem, jointsLength, joints, jointPosesLength, jointPoses, result))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_GetRelativeJoint(This, joint, referenceJoint, result) \
    ((This)->lpVtbl->GetRelativeJoint(This, joint, referenceJoint, result))

#define __x_ABI_CWindows_CPerception_CPeople_CIHandPose_GetRelativeJoints(This, jointsLength, joints, referenceJointsLength, referenceJoints, jointPosesLength, jointPoses) \
    ((This)->lpVtbl->GetRelativeJoints(This, jointsLength, joints, referenceJointsLength, referenceJoints, jointPosesLength, jointPoses))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIHandPose;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIHandPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Perception.People.IHeadPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Perception.People.HeadPose
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_People_IHeadPose[] = L"Windows.Perception.People.IHeadPose";
typedef struct __x_ABI_CWindows_CPerception_CPeople_CIHeadPoseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_ForwardDirection)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_UpDirection)(__x_ABI_CWindows_CPerception_CPeople_CIHeadPose* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CPeople_CIHeadPoseVtbl;

interface __x_ABI_CWindows_CPerception_CPeople_CIHeadPose
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CPeople_CIHeadPoseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_get_ForwardDirection(This, value) \
    ((This)->lpVtbl->get_ForwardDirection(This, value))

#define __x_ABI_CWindows_CPerception_CPeople_CIHeadPose_get_UpDirection(This, value) \
    ((This)->lpVtbl->get_UpDirection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CPeople_CIHeadPose;
#endif /* !defined(____x_ABI_CWindows_CPerception_CPeople_CIHeadPose_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Perception.People.EyesPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.People.IEyesPoseStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IEyesPose ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Perception_People_EyesPose_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_EyesPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_EyesPose[] = L"Windows.Perception.People.EyesPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Perception.People.HandMeshObserver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IHandMeshObserver ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Perception_People_HandMeshObserver_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_HandMeshObserver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_HandMeshObserver[] = L"Windows.Perception.People.HandMeshObserver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Perception.People.HandMeshVertexState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IHandMeshVertexState ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Perception_People_HandMeshVertexState_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_HandMeshVertexState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_HandMeshVertexState[] = L"Windows.Perception.People.HandMeshVertexState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Perception.People.HandPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IHandPose ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Perception_People_HandPose_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_HandPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_HandPose[] = L"Windows.Perception.People.HandPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Perception.People.HeadPose
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Perception.People.IHeadPose ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Perception_People_HeadPose_DEFINED
#define RUNTIMECLASS_Windows_Perception_People_HeadPose_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_People_HeadPose[] = L"Windows.Perception.People.HeadPose";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eperception2Epeople_p_h__

#endif // __windows2Eperception2Epeople_h__
