
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
#ifndef __windows2Egaming2Einput2Eforcefeedback_h__
#define __windows2Egaming2Einput2Eforcefeedback_h__
#ifndef __windows2Egaming2Einput2Eforcefeedback_p_h__
#define __windows2Egaming2Einput2Eforcefeedback_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IConditionForceEffect;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect ABI::Windows::Gaming::Input::ForceFeedback::IConditionForceEffect

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IConditionForceEffectFactory;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory ABI::Windows::Gaming::Input::ForceFeedback::IConditionForceEffectFactory

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IConstantForceEffect;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect ABI::Windows::Gaming::Input::ForceFeedback::IConstantForceEffect

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IForceFeedbackEffect;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackEffect

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IForceFeedbackMotor;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackMotor

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IPeriodicForceEffect;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect ABI::Windows::Gaming::Input::ForceFeedback::IPeriodicForceEffect

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IPeriodicForceEffectFactory;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory ABI::Windows::Gaming::Input::ForceFeedback::IPeriodicForceEffectFactory

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    interface IRampForceEffect;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect ABI::Windows::Gaming::Input::ForceFeedback::IRampForceEffect

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_FWD_DEFINED__

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
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    typedef enum ForceFeedbackLoadEffectResult : int ForceFeedbackLoadEffectResult;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_USE
#define DEF___FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("21f834fc-e845-5ab9-bf85-9534e2397798"))
IAsyncOperation<enum ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackLoadEffectResult> : IAsyncOperation_impl<enum ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackLoadEffectResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackLoadEffectResult> __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_t;
#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f8220a41-f738-51e8-89ba-76bbd66158cb"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackLoadEffectResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackLoadEffectResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackLoadEffectResult> __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    typedef enum ConditionForceEffectKind : int ConditionForceEffectKind;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    typedef enum ForceFeedbackEffectAxes : unsigned int ForceFeedbackEffectAxes;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    typedef enum ForceFeedbackEffectState : int ForceFeedbackEffectState;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    typedef enum PeriodicForceEffectKind : int PeriodicForceEffectKind;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    class ConditionForceEffect;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    class PeriodicForceEffect;
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    enum ConditionForceEffectKind : int
                    {
                        ConditionForceEffectKind_Spring = 0,
                        ConditionForceEffectKind_Damper = 1,
                        ConditionForceEffectKind_Inertia = 2,
                        ConditionForceEffectKind_Friction = 3,
                    };
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    enum ForceFeedbackEffectAxes : unsigned int
                    {
                        ForceFeedbackEffectAxes_None = 0,
                        ForceFeedbackEffectAxes_X = 0x1,
                        ForceFeedbackEffectAxes_Y = 0x2,
                        ForceFeedbackEffectAxes_Z = 0x4,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(ForceFeedbackEffectAxes)
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    enum ForceFeedbackEffectState : int
                    {
                        ForceFeedbackEffectState_Stopped = 0,
                        ForceFeedbackEffectState_Running = 1,
                        ForceFeedbackEffectState_Paused = 2,
                        ForceFeedbackEffectState_Faulted = 3,
                    };
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    enum ForceFeedbackLoadEffectResult : int
                    {
                        ForceFeedbackLoadEffectResult_Succeeded = 0,
                        ForceFeedbackLoadEffectResult_EffectStorageFull = 1,
                        ForceFeedbackLoadEffectResult_EffectNotSupported = 2,
                    };
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    enum PeriodicForceEffectKind : int
                    {
                        PeriodicForceEffectKind_SquareWave = 0,
                        PeriodicForceEffectKind_SineWave = 1,
                        PeriodicForceEffectKind_TriangleWave = 2,
                        PeriodicForceEffectKind_SawtoothWaveUp = 3,
                        PeriodicForceEffectKind_SawtoothWaveDown = 4,
                    };
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IConditionForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.ConditionForceEffect
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IConditionForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.IConditionForceEffect";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    MIDL_INTERFACE("32d1ea68-3695-4e69-85c0-cd1944189140")
                    IConditionForceEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::Gaming::Input::ForceFeedback::ConditionForceEffectKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetParameters(
                            ABI::Windows::Foundation::Numerics::Vector3 direction,
                            FLOAT positiveCoefficient,
                            FLOAT negativeCoefficient,
                            FLOAT maxPositiveMagnitude,
                            FLOAT maxNegativeMagnitude,
                            FLOAT deadZone,
                            FLOAT bias
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IConditionForceEffect = __uuidof(IConditionForceEffect);
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IConditionForceEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.ConditionForceEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IConditionForceEffectFactory[] = L"Windows.Gaming.Input.ForceFeedback.IConditionForceEffectFactory";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    MIDL_INTERFACE("91a99264-1810-4eb6-a773-bfd3b8cddbab")
                    IConditionForceEffectFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Gaming::Input::ForceFeedback::ConditionForceEffectKind effectKind,
                            ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackEffect** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IConditionForceEffectFactory = __uuidof(IConditionForceEffectFactory);
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IConstantForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.ConstantForceEffect
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IConstantForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.IConstantForceEffect";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    MIDL_INTERFACE("9bfa0140-f3c7-415c-b068-0f068734bce0")
                    IConstantForceEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetParameters(
                            ABI::Windows::Foundation::Numerics::Vector3 vector,
                            ABI::Windows::Foundation::TimeSpan duration
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetParametersWithEnvelope(
                            ABI::Windows::Foundation::Numerics::Vector3 vector,
                            FLOAT attackGain,
                            FLOAT sustainGain,
                            FLOAT releaseGain,
                            ABI::Windows::Foundation::TimeSpan startDelay,
                            ABI::Windows::Foundation::TimeSpan attackDuration,
                            ABI::Windows::Foundation::TimeSpan sustainDuration,
                            ABI::Windows::Foundation::TimeSpan releaseDuration,
                            UINT32 repeatCount
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IConstantForceEffect = __uuidof(IConstantForceEffect);
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IForceFeedbackEffect[] = L"Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    MIDL_INTERFACE("a17fba0c-2ae4-48c2-8063-eabd0777cb89")
                    IForceFeedbackEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Gain(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Gain(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_State(
                            ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackEffectState* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IForceFeedbackEffect = __uuidof(IForceFeedbackEffect);
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IForceFeedbackMotor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IForceFeedbackMotor[] = L"Windows.Gaming.Input.ForceFeedback.IForceFeedbackMotor";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    MIDL_INTERFACE("8d3d417c-a5ea-4516-8026-2b00f74ef6e5")
                    IForceFeedbackMotor : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AreEffectsPaused(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MasterGain(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MasterGain(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedAxes(
                            ABI::Windows::Gaming::Input::ForceFeedback::ForceFeedbackEffectAxes* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadEffectAsync(
                            ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackEffect* effect,
                            __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult** asyncOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PauseAllEffects(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResumeAllEffects(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StopAllEffects(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryDisableAsync(
                            __FIAsyncOperation_1_boolean** asyncOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryEnableAsync(
                            __FIAsyncOperation_1_boolean** asyncOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryResetAsync(
                            __FIAsyncOperation_1_boolean** asyncOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryUnloadEffectAsync(
                            ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackEffect* effect,
                            __FIAsyncOperation_1_boolean** asyncOperation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IForceFeedbackMotor = __uuidof(IForceFeedbackMotor);
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IPeriodicForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffect";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    MIDL_INTERFACE("5c5138d7-fc75-4d52-9a0a-efe4cab5fe64")
                    IPeriodicForceEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::Gaming::Input::ForceFeedback::PeriodicForceEffectKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetParameters(
                            ABI::Windows::Foundation::Numerics::Vector3 vector,
                            FLOAT frequency,
                            FLOAT phase,
                            FLOAT bias,
                            ABI::Windows::Foundation::TimeSpan duration
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetParametersWithEnvelope(
                            ABI::Windows::Foundation::Numerics::Vector3 vector,
                            FLOAT frequency,
                            FLOAT phase,
                            FLOAT bias,
                            FLOAT attackGain,
                            FLOAT sustainGain,
                            FLOAT releaseGain,
                            ABI::Windows::Foundation::TimeSpan startDelay,
                            ABI::Windows::Foundation::TimeSpan attackDuration,
                            ABI::Windows::Foundation::TimeSpan sustainDuration,
                            ABI::Windows::Foundation::TimeSpan releaseDuration,
                            UINT32 repeatCount
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPeriodicForceEffect = __uuidof(IPeriodicForceEffect);
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IPeriodicForceEffectFactory[] = L"Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffectFactory";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    MIDL_INTERFACE("6f62eb1a-9851-477b-b318-35ecaa15070f")
                    IPeriodicForceEffectFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Gaming::Input::ForceFeedback::PeriodicForceEffectKind effectKind,
                            ABI::Windows::Gaming::Input::ForceFeedback::IForceFeedbackEffect** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPeriodicForceEffectFactory = __uuidof(IPeriodicForceEffectFactory);
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IRampForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.RampForceEffect
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IRampForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.IRampForceEffect";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace ForceFeedback {
                    MIDL_INTERFACE("f1f81259-1ca6-4080-b56d-b43f3354d052")
                    IRampForceEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetParameters(
                            ABI::Windows::Foundation::Numerics::Vector3 startVector,
                            ABI::Windows::Foundation::Numerics::Vector3 endVector,
                            ABI::Windows::Foundation::TimeSpan duration
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetParametersWithEnvelope(
                            ABI::Windows::Foundation::Numerics::Vector3 startVector,
                            ABI::Windows::Foundation::Numerics::Vector3 endVector,
                            FLOAT attackGain,
                            FLOAT sustainGain,
                            FLOAT releaseGain,
                            ABI::Windows::Foundation::TimeSpan startDelay,
                            ABI::Windows::Foundation::TimeSpan attackDuration,
                            ABI::Windows::Foundation::TimeSpan sustainDuration,
                            ABI::Windows::Foundation::TimeSpan releaseDuration,
                            UINT32 repeatCount
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRampForceEffect = __uuidof(IRampForceEffect);
                } /* ForceFeedback */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.ConditionForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Gaming.Input.ForceFeedback.IConditionForceEffectFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect ** Default Interface **
 *    Windows.Gaming.Input.ForceFeedback.IConditionForceEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ConditionForceEffect_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ConditionForceEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_ConditionForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.ConstantForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect ** Default Interface **
 *    Windows.Gaming.Input.ForceFeedback.IConstantForceEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ConstantForceEffect_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ConstantForceEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_ConstantForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackMotor ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ForceFeedbackMotor_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ForceFeedbackMotor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_ForceFeedbackMotor[] = L"Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffectFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect ** Default Interface **
 *    Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_PeriodicForceEffect_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_PeriodicForceEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_PeriodicForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.RampForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect ** Default Interface **
 *    Windows.Gaming.Input.ForceFeedback.IRampForceEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_RampForceEffect_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_RampForceEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_RampForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.RampForceEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect;

#endif // ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_FWD_DEFINED__

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

typedef enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackLoadEffectResult __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackLoadEffectResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult;

typedef struct __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackLoadEffectResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResultVtbl;

interface __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* This,
        __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CConditionForceEffectKind __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CConditionForceEffectKind;

typedef enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackEffectAxes __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackEffectAxes;

typedef enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackEffectState __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackEffectState;

typedef enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CPeriodicForceEffectKind __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CPeriodicForceEffectKind;

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CConditionForceEffectKind
{
    ConditionForceEffectKind_Spring = 0,
    ConditionForceEffectKind_Damper = 1,
    ConditionForceEffectKind_Inertia = 2,
    ConditionForceEffectKind_Friction = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackEffectAxes
{
    ForceFeedbackEffectAxes_None = 0,
    ForceFeedbackEffectAxes_X = 0x1,
    ForceFeedbackEffectAxes_Y = 0x2,
    ForceFeedbackEffectAxes_Z = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackEffectState
{
    ForceFeedbackEffectState_Stopped = 0,
    ForceFeedbackEffectState_Running = 1,
    ForceFeedbackEffectState_Paused = 2,
    ForceFeedbackEffectState_Faulted = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackLoadEffectResult
{
    ForceFeedbackLoadEffectResult_Succeeded = 0,
    ForceFeedbackLoadEffectResult_EffectStorageFull = 1,
    ForceFeedbackLoadEffectResult_EffectNotSupported = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CPeriodicForceEffectKind
{
    PeriodicForceEffectKind_SquareWave = 0,
    PeriodicForceEffectKind_SineWave = 1,
    PeriodicForceEffectKind_TriangleWave = 2,
    PeriodicForceEffectKind_SawtoothWaveUp = 3,
    PeriodicForceEffectKind_SawtoothWaveDown = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IConditionForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.ConditionForceEffect
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IConditionForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.IConditionForceEffect";
typedef struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect* This,
        enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CConditionForceEffectKind* value);
    HRESULT (STDMETHODCALLTYPE* SetParameters)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 direction,
        FLOAT positiveCoefficient,
        FLOAT negativeCoefficient,
        FLOAT maxPositiveMagnitude,
        FLOAT maxNegativeMagnitude,
        FLOAT deadZone,
        FLOAT bias);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_SetParameters(This, direction, positiveCoefficient, negativeCoefficient, maxPositiveMagnitude, maxNegativeMagnitude, deadZone, bias) \
    ((This)->lpVtbl->SetParameters(This, direction, positiveCoefficient, negativeCoefficient, maxPositiveMagnitude, maxNegativeMagnitude, deadZone, bias))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IConditionForceEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.ConditionForceEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IConditionForceEffectFactory[] = L"Windows.Gaming.Input.ForceFeedback.IConditionForceEffectFactory";
typedef struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory* This,
        enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CConditionForceEffectKind effectKind,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactoryVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_CreateInstance(This, effectKind, value) \
    ((This)->lpVtbl->CreateInstance(This, effectKind, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConditionForceEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IConstantForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.ConstantForceEffect
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IConstantForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.IConstantForceEffect";
typedef struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetParameters)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 vector,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration);
    HRESULT (STDMETHODCALLTYPE* SetParametersWithEnvelope)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 vector,
        FLOAT attackGain,
        FLOAT sustainGain,
        FLOAT releaseGain,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan startDelay,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan attackDuration,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan sustainDuration,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan releaseDuration,
        UINT32 repeatCount);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffectVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_SetParameters(This, vector, duration) \
    ((This)->lpVtbl->SetParameters(This, vector, duration))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_SetParametersWithEnvelope(This, vector, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount) \
    ((This)->lpVtbl->SetParametersWithEnvelope(This, vector, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIConstantForceEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IForceFeedbackEffect[] = L"Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect";
typedef struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Gain)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Gain)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This,
        enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackEffectState* value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* This);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffectVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_get_Gain(This, value) \
    ((This)->lpVtbl->get_Gain(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_put_Gain(This, value) \
    ((This)->lpVtbl->put_Gain(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IForceFeedbackMotor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IForceFeedbackMotor[] = L"Windows.Gaming.Input.ForceFeedback.IForceFeedbackMotor";
typedef struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AreEffectsPaused)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MasterGain)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_MasterGain)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedAxes)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CForceFeedbackEffectAxes* value);
    HRESULT (STDMETHODCALLTYPE* LoadEffectAsync)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* effect,
        __FIAsyncOperation_1_Windows__CGaming__CInput__CForceFeedback__CForceFeedbackLoadEffectResult** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* PauseAllEffects)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This);
    HRESULT (STDMETHODCALLTYPE* ResumeAllEffects)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This);
    HRESULT (STDMETHODCALLTYPE* StopAllEffects)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This);
    HRESULT (STDMETHODCALLTYPE* TryDisableAsync)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        __FIAsyncOperation_1_boolean** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* TryEnableAsync)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        __FIAsyncOperation_1_boolean** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* TryResetAsync)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        __FIAsyncOperation_1_boolean** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* TryUnloadEffectAsync)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor* This,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect* effect,
        __FIAsyncOperation_1_boolean** asyncOperation);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotorVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_get_AreEffectsPaused(This, value) \
    ((This)->lpVtbl->get_AreEffectsPaused(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_get_MasterGain(This, value) \
    ((This)->lpVtbl->get_MasterGain(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_put_MasterGain(This, value) \
    ((This)->lpVtbl->put_MasterGain(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_get_SupportedAxes(This, value) \
    ((This)->lpVtbl->get_SupportedAxes(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_LoadEffectAsync(This, effect, asyncOperation) \
    ((This)->lpVtbl->LoadEffectAsync(This, effect, asyncOperation))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_PauseAllEffects(This) \
    ((This)->lpVtbl->PauseAllEffects(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_ResumeAllEffects(This) \
    ((This)->lpVtbl->ResumeAllEffects(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_StopAllEffects(This) \
    ((This)->lpVtbl->StopAllEffects(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_TryDisableAsync(This, asyncOperation) \
    ((This)->lpVtbl->TryDisableAsync(This, asyncOperation))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_TryEnableAsync(This, asyncOperation) \
    ((This)->lpVtbl->TryEnableAsync(This, asyncOperation))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_TryResetAsync(This, asyncOperation) \
    ((This)->lpVtbl->TryResetAsync(This, asyncOperation))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_TryUnloadEffectAsync(This, effect, asyncOperation) \
    ((This)->lpVtbl->TryUnloadEffectAsync(This, effect, asyncOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackMotor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IPeriodicForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffect";
typedef struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This,
        enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CPeriodicForceEffectKind* value);
    HRESULT (STDMETHODCALLTYPE* SetParameters)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 vector,
        FLOAT frequency,
        FLOAT phase,
        FLOAT bias,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration);
    HRESULT (STDMETHODCALLTYPE* SetParametersWithEnvelope)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 vector,
        FLOAT frequency,
        FLOAT phase,
        FLOAT bias,
        FLOAT attackGain,
        FLOAT sustainGain,
        FLOAT releaseGain,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan startDelay,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan attackDuration,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan sustainDuration,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan releaseDuration,
        UINT32 repeatCount);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_SetParameters(This, vector, frequency, phase, bias, duration) \
    ((This)->lpVtbl->SetParameters(This, vector, frequency, phase, bias, duration))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_SetParametersWithEnvelope(This, vector, frequency, phase, bias, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount) \
    ((This)->lpVtbl->SetParametersWithEnvelope(This, vector, frequency, phase, bias, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IPeriodicForceEffectFactory[] = L"Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffectFactory";
typedef struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory* This,
        enum __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CPeriodicForceEffectKind effectKind,
        __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIForceFeedbackEffect** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactoryVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_CreateInstance(This, effectKind, value) \
    ((This)->lpVtbl->CreateInstance(This, effectKind, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIPeriodicForceEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.ForceFeedback.IRampForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.ForceFeedback.RampForceEffect
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_ForceFeedback_IRampForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.IRampForceEffect";
typedef struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetParameters)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 startVector,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 endVector,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration);
    HRESULT (STDMETHODCALLTYPE* SetParametersWithEnvelope)(__x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 startVector,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 endVector,
        FLOAT attackGain,
        FLOAT sustainGain,
        FLOAT releaseGain,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan startDelay,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan attackDuration,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan sustainDuration,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan releaseDuration,
        UINT32 repeatCount);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffectVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_SetParameters(This, startVector, endVector, duration) \
    ((This)->lpVtbl->SetParameters(This, startVector, endVector, duration))

#define __x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_SetParametersWithEnvelope(This, startVector, endVector, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount) \
    ((This)->lpVtbl->SetParametersWithEnvelope(This, startVector, endVector, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CForceFeedback_CIRampForceEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.ConditionForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Gaming.Input.ForceFeedback.IConditionForceEffectFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect ** Default Interface **
 *    Windows.Gaming.Input.ForceFeedback.IConditionForceEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ConditionForceEffect_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ConditionForceEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_ConditionForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.ConstantForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect ** Default Interface **
 *    Windows.Gaming.Input.ForceFeedback.IConstantForceEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ConstantForceEffect_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ConstantForceEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_ConstantForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackMotor ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ForceFeedbackMotor_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_ForceFeedbackMotor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_ForceFeedbackMotor[] = L"Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffectFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect ** Default Interface **
 *    Windows.Gaming.Input.ForceFeedback.IPeriodicForceEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_PeriodicForceEffect_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_PeriodicForceEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_PeriodicForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.ForceFeedback.RampForceEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect ** Default Interface **
 *    Windows.Gaming.Input.ForceFeedback.IRampForceEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_RampForceEffect_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_ForceFeedback_RampForceEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_ForceFeedback_RampForceEffect[] = L"Windows.Gaming.Input.ForceFeedback.RampForceEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egaming2Einput2Eforcefeedback_p_h__

#endif // __windows2Egaming2Einput2Eforcefeedback_h__
