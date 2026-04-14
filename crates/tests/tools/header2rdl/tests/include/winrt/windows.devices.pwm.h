
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
#ifndef __windows2Edevices2Epwm_h__
#define __windows2Edevices2Epwm_h__
#ifndef __windows2Edevices2Epwm_p_h__
#define __windows2Edevices2Epwm_p_h__


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
#if !defined(WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION)
#define WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION)

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
#include "Windows.Devices.h"
#include "Windows.Devices.Pwm.Provider.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                interface IPwmController;
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController ABI::Windows::Devices::Pwm::IPwmController

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                interface IPwmControllerStatics;
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics ABI::Windows::Devices::Pwm::IPwmControllerStatics

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                interface IPwmControllerStatics2;
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2 ABI::Windows::Devices::Pwm::IPwmControllerStatics2

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                interface IPwmControllerStatics3;
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3 ABI::Windows::Devices::Pwm::IPwmControllerStatics3

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                interface IPwmPin;
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin ABI::Windows::Devices::Pwm::IPwmPin

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                class PwmController;
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0a288d41-1f20-5d16-85dd-52855b11569a"))
IAsyncOperation<ABI::Windows::Devices::Pwm::PwmController*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Pwm::PwmController*, ABI::Windows::Devices::Pwm::IPwmController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Pwm.PwmController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Pwm::PwmController*> __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_t;
#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5fc68e9f-ffff-5d53-ba21-9c33ef56b240"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Pwm::PwmController*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Pwm::PwmController*, ABI::Windows::Devices::Pwm::IPwmController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Pwm.PwmController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Pwm::PwmController*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CPwm__CPwmController_USE
#define DEF___FIIterator_1_Windows__CDevices__CPwm__CPwmController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("599330bd-b0ca-533e-938f-5dd4242bf513"))
IIterator<ABI::Windows::Devices::Pwm::PwmController*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Pwm::PwmController*, ABI::Windows::Devices::Pwm::IPwmController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Pwm.PwmController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Pwm::PwmController*> __FIIterator_1_Windows__CDevices__CPwm__CPwmController_t;
#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CPwm__CPwmController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CPwm__CPwmController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CPwm__CPwmController_USE
#define DEF___FIIterable_1_Windows__CDevices__CPwm__CPwmController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1403a6ab-73cb-5805-9bbc-a0dd39d476b0"))
IIterable<ABI::Windows::Devices::Pwm::PwmController*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Pwm::PwmController*, ABI::Windows::Devices::Pwm::IPwmController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Pwm.PwmController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Pwm::PwmController*> __FIIterable_1_Windows__CDevices__CPwm__CPwmController_t;
#define __FIIterable_1_Windows__CDevices__CPwm__CPwmController ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CPwm__CPwmController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CPwm__CPwmController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE
#define DEF___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("07cb8bac-3bac-5ea0-919a-9eaf620270ae"))
IVectorView<ABI::Windows::Devices::Pwm::PwmController*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Pwm::PwmController*, ABI::Windows::Devices::Pwm::IPwmController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Pwm.PwmController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Pwm::PwmController*> __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_t;
#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CPwm__CPwmController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e4151e8d-4688-5023-9f5d-008bbd904891"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CPwm__CPwmController*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CPwm__CPwmController*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Pwm.PwmController>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CPwm__CPwmController*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e72bd078-ce02-55ac-a7b9-abd01248d888"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CPwm__CPwmController*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CPwm__CPwmController*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Pwm.PwmController>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CPwm__CPwmController*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                namespace Provider {
                    interface IPwmProvider;
                } /* Provider */
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider ABI::Windows::Devices::Pwm::Provider::IPwmProvider

#endif // ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider_FWD_DEFINED__

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
        namespace Devices {
            namespace Pwm {
                typedef enum PwmPulsePolarity : int PwmPulsePolarity;
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                class PwmPin;
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Pwm.PwmPulsePolarity
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                enum PwmPulsePolarity : int
                {
                    PwmPulsePolarity_ActiveHigh = 0,
                    PwmPulsePolarity_ActiveLow = 1,
                };
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmController[] = L"Windows.Devices.Pwm.IPwmController";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                MIDL_INTERFACE("c45f5c85-d2e8-42cf-9bd6-cf5ed029e6a7")
                IPwmController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PinCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActualFrequency(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDesiredFrequency(
                        DOUBLE desiredFrequency,
                        DOUBLE* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinFrequency(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxFrequency(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenPin(
                        INT32 pinNumber,
                        ABI::Windows::Devices::Pwm::IPwmPin** pin
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPwmController = __uuidof(IPwmController);
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmControllerStatics[] = L"Windows.Devices.Pwm.IPwmControllerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                MIDL_INTERFACE("4263bda1-8946-4404-bd48-81dd124af4d9")
                IPwmControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetControllersAsync(
                        ABI::Windows::Devices::Pwm::Provider::IPwmProvider* provider,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPwmControllerStatics = __uuidof(IPwmControllerStatics);
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmControllerStatics2
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmControllerStatics2[] = L"Windows.Devices.Pwm.IPwmControllerStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                MIDL_INTERFACE("44fc5b1f-f119-4bdd-97ad-f76ef986736d")
                IPwmControllerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPwmControllerStatics2 = __uuidof(IPwmControllerStatics2);
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmControllerStatics3
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmControllerStatics3[] = L"Windows.Devices.Pwm.IPwmControllerStatics3";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                MIDL_INTERFACE("b2581871-0229-4344-ae3f-9b7cd0e66b94")
                IPwmControllerStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromFriendlyName(
                        HSTRING friendlyName,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPwmControllerStatics3 = __uuidof(IPwmControllerStatics3);
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmPin
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmPin
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmPin[] = L"Windows.Devices.Pwm.IPwmPin";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Pwm {
                MIDL_INTERFACE("22972dc8-c6cf-4821-b7f9-c6454fb6af79")
                IPwmPin : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Controller(
                        ABI::Windows::Devices::Pwm::IPwmController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetActiveDutyCyclePercentage(
                        DOUBLE* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetActiveDutyCyclePercentage(
                        DOUBLE dutyCyclePercentage
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Polarity(
                        ABI::Windows::Devices::Pwm::PwmPulsePolarity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Polarity(
                        ABI::Windows::Devices::Pwm::PwmPulsePolarity value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsStarted(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPwmPin = __uuidof(IPwmPin);
            } /* Pwm */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmPin;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Pwm.PwmController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Pwm.IPwmControllerStatics interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *   Static Methods exist on the Windows.Devices.Pwm.IPwmControllerStatics2 interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *   Static Methods exist on the Windows.Devices.Pwm.IPwmControllerStatics3 interface starting with version 3.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Pwm.IPwmController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Pwm_PwmController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Pwm_PwmController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Pwm_PwmController[] = L"Windows.Devices.Pwm.PwmController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Pwm.PwmPin
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Pwm.IPwmPin ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Pwm_PwmPin_DEFINED
#define RUNTIMECLASS_Windows_Devices_Pwm_PwmPin_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Pwm_PwmPin[] = L"Windows.Devices.Pwm.PwmPin";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPwm_CIPwmController __x_ABI_CWindows_CDevices_CPwm_CIPwmController;

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics;

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2 __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2;

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3 __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3;

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPwm_CIPwmPin __x_ABI_CWindows_CDevices_CPwm_CIPwmPin;

#endif // ____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* This,
        __x_ABI_CWindows_CDevices_CPwm_CIPwmController** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmControllerVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController* This,
        __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CPwm__CPwmController __FIIterator_1_Windows__CDevices__CPwm__CPwmController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CPwm__CPwmController;

typedef struct __FIIterator_1_Windows__CDevices__CPwm__CPwmControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This,
        __x_ABI_CWindows_CDevices_CPwm_CIPwmController** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CPwm__CPwmController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CPwm_CIPwmController** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CPwm__CPwmControllerVtbl;

interface __FIIterator_1_Windows__CDevices__CPwm__CPwmController
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CPwm__CPwmControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CPwm__CPwmController_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CPwm__CPwmController __FIIterable_1_Windows__CDevices__CPwm__CPwmController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CPwm__CPwmController;

typedef struct __FIIterable_1_Windows__CDevices__CPwm__CPwmControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CPwm__CPwmController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CPwm__CPwmController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CPwm__CPwmController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CPwm__CPwmController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CPwm__CPwmController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CPwm__CPwmController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CPwm__CPwmController* This,
        __FIIterator_1_Windows__CDevices__CPwm__CPwmController** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CPwm__CPwmControllerVtbl;

interface __FIIterable_1_Windows__CDevices__CPwm__CPwmController
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CPwm__CPwmControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CPwm__CPwmController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CPwm__CPwmController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CPwm__CPwmController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CPwm__CPwmController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CPwm__CPwmController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CPwm__CPwmController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CPwm__CPwmController_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CPwm__CPwmController __FIVectorView_1_Windows__CDevices__CPwm__CPwmController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CPwm__CPwmController;

typedef struct __FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CPwm_CIPwmController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        __x_ABI_CWindows_CDevices_CPwm_CIPwmController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CPwm_CIPwmController** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl;

interface __FIVectorView_1_Windows__CDevices__CPwm__CPwmController
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        __FIVectorView_1_Windows__CDevices__CPwm__CPwmController** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider;

#endif // ____x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CPwm_CPwmPulsePolarity __x_ABI_CWindows_CDevices_CPwm_CPwmPulsePolarity;

/*
 *
 * Struct Windows.Devices.Pwm.PwmPulsePolarity
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CPwm_CPwmPulsePolarity
{
    PwmPulsePolarity_ActiveHigh = 0,
    PwmPulsePolarity_ActiveLow = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmController[] = L"Windows.Devices.Pwm.IPwmController";
typedef struct __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PinCount)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ActualFrequency)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* SetDesiredFrequency)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        DOUBLE desiredFrequency,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_MinFrequency)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxFrequency)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* OpenPin)(__x_ABI_CWindows_CDevices_CPwm_CIPwmController* This,
        INT32 pinNumber,
        __x_ABI_CWindows_CDevices_CPwm_CIPwmPin** pin);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerVtbl;

interface __x_ABI_CWindows_CDevices_CPwm_CIPwmController
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_get_PinCount(This, value) \
    ((This)->lpVtbl->get_PinCount(This, value))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_get_ActualFrequency(This, value) \
    ((This)->lpVtbl->get_ActualFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_SetDesiredFrequency(This, desiredFrequency, result) \
    ((This)->lpVtbl->SetDesiredFrequency(This, desiredFrequency, result))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_get_MinFrequency(This, value) \
    ((This)->lpVtbl->get_MinFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_get_MaxFrequency(This, value) \
    ((This)->lpVtbl->get_MaxFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmController_OpenPin(This, pinNumber, pin) \
    ((This)->lpVtbl->OpenPin(This, pinNumber, pin))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmControllerStatics[] = L"Windows.Devices.Pwm.IPwmControllerStatics";
typedef struct __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetControllersAsync)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics* This,
        __x_ABI_CWindows_CDevices_CPwm_CProvider_CIPwmProvider* provider,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CPwm__CPwmController** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_GetControllersAsync(This, provider, operation) \
    ((This)->lpVtbl->GetControllersAsync(This, provider, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmControllerStatics2
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmControllerStatics2[] = L"Windows.Devices.Pwm.IPwmControllerStatics2";
typedef struct __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2* This,
        __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_GetDefaultAsync(This, operation) \
    ((This)->lpVtbl->GetDefaultAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmControllerStatics3
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmControllerStatics3[] = L"Windows.Devices.Pwm.IPwmControllerStatics3";
typedef struct __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromFriendlyName)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This,
        HSTRING friendlyName,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CPwm__CPwmController** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3Vtbl;

interface __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_GetDeviceSelectorFromFriendlyName(This, friendlyName, result) \
    ((This)->lpVtbl->GetDeviceSelectorFromFriendlyName(This, friendlyName, result))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_FromIdAsync(This, deviceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmControllerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Pwm.IPwmPin
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Pwm.PwmPin
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Pwm_IPwmPin[] = L"Windows.Devices.Pwm.IPwmPin";
typedef struct __x_ABI_CWindows_CDevices_CPwm_CIPwmPinVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Controller)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        __x_ABI_CWindows_CDevices_CPwm_CIPwmController** value);
    HRESULT (STDMETHODCALLTYPE* GetActiveDutyCyclePercentage)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* SetActiveDutyCyclePercentage)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        DOUBLE dutyCyclePercentage);
    HRESULT (STDMETHODCALLTYPE* get_Polarity)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        enum __x_ABI_CWindows_CDevices_CPwm_CPwmPulsePolarity* value);
    HRESULT (STDMETHODCALLTYPE* put_Polarity)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        enum __x_ABI_CWindows_CDevices_CPwm_CPwmPulsePolarity value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This);
    HRESULT (STDMETHODCALLTYPE* get_IsStarted)(__x_ABI_CWindows_CDevices_CPwm_CIPwmPin* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPwm_CIPwmPinVtbl;

interface __x_ABI_CWindows_CDevices_CPwm_CIPwmPin
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPwm_CIPwmPinVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_get_Controller(This, value) \
    ((This)->lpVtbl->get_Controller(This, value))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_GetActiveDutyCyclePercentage(This, result) \
    ((This)->lpVtbl->GetActiveDutyCyclePercentage(This, result))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_SetActiveDutyCyclePercentage(This, dutyCyclePercentage) \
    ((This)->lpVtbl->SetActiveDutyCyclePercentage(This, dutyCyclePercentage))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_get_Polarity(This, value) \
    ((This)->lpVtbl->get_Polarity(This, value))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_put_Polarity(This, value) \
    ((This)->lpVtbl->put_Polarity(This, value))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CDevices_CPwm_CIPwmPin_get_IsStarted(This, value) \
    ((This)->lpVtbl->get_IsStarted(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPwm_CIPwmPin;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPwm_CIPwmPin_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Pwm.PwmController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Pwm.IPwmControllerStatics interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *   Static Methods exist on the Windows.Devices.Pwm.IPwmControllerStatics2 interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *   Static Methods exist on the Windows.Devices.Pwm.IPwmControllerStatics3 interface starting with version 3.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Pwm.IPwmController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Pwm_PwmController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Pwm_PwmController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Pwm_PwmController[] = L"Windows.Devices.Pwm.PwmController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Pwm.PwmPin
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Pwm.IPwmPin ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Pwm_PwmPin_DEFINED
#define RUNTIMECLASS_Windows_Devices_Pwm_PwmPin_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Pwm_PwmPin[] = L"Windows.Devices.Pwm.PwmPin";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Epwm_p_h__

#endif // __windows2Edevices2Epwm_h__
