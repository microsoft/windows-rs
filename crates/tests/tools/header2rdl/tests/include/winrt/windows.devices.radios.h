
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
#ifndef __windows2Edevices2Eradios_h__
#define __windows2Edevices2Eradios_h__
#ifndef __windows2Edevices2Eradios_p_h__
#define __windows2Edevices2Eradios_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                interface IRadio;
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CRadios_CIRadio ABI::Windows::Devices::Radios::IRadio

#endif // ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                interface IRadioStatics;
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics ABI::Windows::Devices::Radios::IRadioStatics

#endif // ____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                class Radio;
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eac62c40-8dbc-5854-8ba0-b7b9940e7389"))
IAsyncOperation<ABI::Windows::Devices::Radios::Radio*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Radios::Radio*, ABI::Windows::Devices::Radios::IRadio*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Radios.Radio>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Radios::Radio*> __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8a5c7e3a-80e2-585b-8630-7a8e777f0354"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Radios::Radio*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Radios::Radio*, ABI::Windows::Devices::Radios::IRadio*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Radios.Radio>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Radios::Radio*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                typedef enum RadioAccessStatus : int RadioAccessStatus;
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("21fb30ef-072f-502c-9898-d0c3b2cd9ac5"))
IAsyncOperation<enum ABI::Windows::Devices::Radios::RadioAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Devices::Radios::RadioAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Radios.RadioAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Devices::Radios::RadioAccessStatus> __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bd248e73-f05f-574c-ae3d-9b95c4bf282a"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Radios::RadioAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Devices::Radios::RadioAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Radios.RadioAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Radios::RadioAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIIterator_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cf37ede7-eaec-5b8f-ad31-4d51abd9db05"))
IIterator<ABI::Windows::Devices::Radios::Radio*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Radios::Radio*, ABI::Windows::Devices::Radios::IRadio*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Radios.Radio>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Radios::Radio*> __FIIterator_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIIterator_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CRadios__CRadio_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIIterable_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e82500af-1f53-504e-b8be-dac4fbb69084"))
IIterable<ABI::Windows::Devices::Radios::Radio*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Radios::Radio*, ABI::Windows::Devices::Radios::IRadio*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Radios.Radio>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Radios::Radio*> __FIIterable_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIIterable_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CRadios__CRadio_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("65066c36-090b-5466-b8e5-e7565dc34175"))
IVectorView<ABI::Windows::Devices::Radios::Radio*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Radios::Radio*, ABI::Windows::Devices::Radios::IRadio*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Radios.Radio>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Radios::Radio*> __FIVectorView_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CRadios__CRadio_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("040b54a1-203e-58f5-943f-c1cca86bd532"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CRadios__CRadio*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CRadios__CRadio*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Radios.Radio>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CRadios__CRadio*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d30691e6-60a0-59c9-8965-5bbe282e8208"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CRadios__CRadio*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CRadios__CRadio*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Radios.Radio>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CRadios__CRadio*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fc6aa329-b586-5ebb-9e85-3f6b84ebdf18"))
ITypedEventHandler<ABI::Windows::Devices::Radios::Radio*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Radios::Radio*, ABI::Windows::Devices::Radios::IRadio*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Radios.Radio, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Radios::Radio*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                typedef enum RadioKind : int RadioKind;
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                typedef enum RadioState : int RadioState;
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Radios.RadioAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                enum RadioAccessStatus : int
                {
                    RadioAccessStatus_Unspecified = 0,
                    RadioAccessStatus_Allowed = 1,
                    RadioAccessStatus_DeniedByUser = 2,
                    RadioAccessStatus_DeniedBySystem = 3,
                };
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Radios.RadioKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                enum RadioKind : int
                {
                    RadioKind_Other = 0,
                    RadioKind_WiFi = 1,
                    RadioKind_MobileBroadband = 2,
                    RadioKind_Bluetooth = 3,
                    RadioKind_FM = 4,
                };
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Radios.RadioState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                enum RadioState : int
                {
                    RadioState_Unknown = 0,
                    RadioState_On = 1,
                    RadioState_Off = 2,
                    RadioState_Disabled = 3,
                };
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Radios.IRadio
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Radios.Radio
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CRadios_CIRadio_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadio_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Radios_IRadio[] = L"Windows.Devices.Radios.IRadio";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                MIDL_INTERFACE("252118df-b33e-416a-875f-1cf38ae2d83e")
                IRadio : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetStateAsync(
                        ABI::Windows::Devices::Radios::RadioState value,
                        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus** retval
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable* handler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Devices::Radios::RadioState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Devices::Radios::RadioKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadio = __uuidof(IRadio);
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CRadios_CIRadio;
#endif /* !defined(____x_ABI_CWindows_CDevices_CRadios_CIRadio_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Radios.IRadioStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Radios.Radio
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Radios_IRadioStatics[] = L"Windows.Devices.Radios.IRadioStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Radios {
                MIDL_INTERFACE("5fb6a12e-67cb-46ae-aae9-65919f86eff4")
                IRadioStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetRadiosAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* deviceSelector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRadioStatics = __uuidof(IRadioStatics);
            } /* Radios */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CRadios_CIRadioStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Radios.Radio
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Radios.IRadioStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Radios.IRadio ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Radios_Radio_DEFINED
#define RUNTIMECLASS_Windows_Devices_Radios_Radio_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Radios_Radio[] = L"Windows.Devices.Radios.Radio";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CRadios_CIRadio __x_ABI_CWindows_CDevices_CRadios_CIRadio;

#endif // ____x_ABI_CWindows_CDevices_CRadios_CIRadio_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics;

#endif // ____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* This,
        __x_ABI_CWindows_CDevices_CRadios_CIRadio** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CRadios_CRadioAccessStatus __x_ABI_CWindows_CDevices_CRadios_CRadioAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        enum __x_ABI_CWindows_CDevices_CRadios_CRadioAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus* This,
        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CRadios__CRadioAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CRadios__CRadio __FIIterator_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIIterator_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This,
        __x_ABI_CWindows_CDevices_CRadios_CIRadio** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CRadios__CRadio* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CRadios_CIRadio** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIIterator_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CRadios__CRadio_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CRadios__CRadio __FIIterable_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIIterable_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CRadios__CRadio* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CRadios__CRadio* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CRadios__CRadio* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CRadios__CRadio* This,
        __FIIterator_1_Windows__CDevices__CRadios__CRadio** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIIterable_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CRadios__CRadio_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CRadios__CRadio_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CRadios__CRadio_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CRadios__CRadio_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CRadios__CRadio __FIVectorView_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CRadios_CIRadio** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        __x_ABI_CWindows_CDevices_CRadios_CIRadio* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CRadios_CIRadio** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIVectorView_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        __FIVectorView_1_Windows__CDevices__CRadios__CRadio** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable* This,
        __x_ABI_CWindows_CDevices_CRadios_CIRadio* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CRadios_CRadioKind __x_ABI_CWindows_CDevices_CRadios_CRadioKind;

typedef enum __x_ABI_CWindows_CDevices_CRadios_CRadioState __x_ABI_CWindows_CDevices_CRadios_CRadioState;

/*
 *
 * Struct Windows.Devices.Radios.RadioAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CRadios_CRadioAccessStatus
{
    RadioAccessStatus_Unspecified = 0,
    RadioAccessStatus_Allowed = 1,
    RadioAccessStatus_DeniedByUser = 2,
    RadioAccessStatus_DeniedBySystem = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Radios.RadioKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CRadios_CRadioKind
{
    RadioKind_Other = 0,
    RadioKind_WiFi = 1,
    RadioKind_MobileBroadband = 2,
    RadioKind_Bluetooth = 3,
    RadioKind_FM = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Radios.RadioState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CRadios_CRadioState
{
    RadioState_Unknown = 0,
    RadioState_On = 1,
    RadioState_Off = 2,
    RadioState_Disabled = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Radios.IRadio
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Radios.Radio
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CRadios_CIRadio_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadio_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Radios_IRadio[] = L"Windows.Devices.Radios.IRadio";
typedef struct __x_ABI_CWindows_CDevices_CRadios_CIRadioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetStateAsync)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        enum __x_ABI_CWindows_CDevices_CRadios_CRadioState value,
        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus** retval);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        __FITypedEventHandler_2_Windows__CDevices__CRadios__CRadio_IInspectable* handler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        enum __x_ABI_CWindows_CDevices_CRadios_CRadioState* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CDevices_CRadios_CIRadio* This,
        enum __x_ABI_CWindows_CDevices_CRadios_CRadioKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CRadios_CIRadioVtbl;

interface __x_ABI_CWindows_CDevices_CRadios_CIRadio
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CRadios_CIRadioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_SetStateAsync(This, value, retval) \
    ((This)->lpVtbl->SetStateAsync(This, value, retval))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_add_StateChanged(This, handler, eventCookie) \
    ((This)->lpVtbl->add_StateChanged(This, handler, eventCookie))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_remove_StateChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_StateChanged(This, eventCookie))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadio_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CRadios_CIRadio;
#endif /* !defined(____x_ABI_CWindows_CDevices_CRadios_CIRadio_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Radios.IRadioStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Radios.Radio
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Radios_IRadioStatics[] = L"Windows.Devices.Radios.IRadioStatics";
typedef struct __x_ABI_CWindows_CDevices_CRadios_CIRadioStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetRadiosAsync)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CRadios__CRadio** value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This,
        HSTRING* deviceSelector);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadio** value);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CDevices_CRadios_CIRadioStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CRadios__CRadioAccessStatus** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CRadios_CIRadioStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CRadios_CIRadioStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_GetRadiosAsync(This, value) \
    ((This)->lpVtbl->GetRadiosAsync(This, value))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_GetDeviceSelector(This, deviceSelector) \
    ((This)->lpVtbl->GetDeviceSelector(This, deviceSelector))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_FromIdAsync(This, deviceId, value) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, value))

#define __x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_RequestAccessAsync(This, value) \
    ((This)->lpVtbl->RequestAccessAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CRadios_CIRadioStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CRadios_CIRadioStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Radios.Radio
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Radios.IRadioStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Radios.IRadio ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Radios_Radio_DEFINED
#define RUNTIMECLASS_Windows_Devices_Radios_Radio_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Radios_Radio[] = L"Windows.Devices.Radios.Radio";
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
#endif // __windows2Edevices2Eradios_p_h__

#endif // __windows2Edevices2Eradios_h__
