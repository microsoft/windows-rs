
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
#ifndef __windows2Edevices2Eadc_h__
#define __windows2Edevices2Eadc_h__
#ifndef __windows2Edevices2Eadc_p_h__
#define __windows2Edevices2Eadc_p_h__


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
#include "Windows.Devices.Adc.Provider.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                interface IAdcChannel;
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel ABI::Windows::Devices::Adc::IAdcChannel

#endif // ____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CIAdcController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                interface IAdcController;
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController ABI::Windows::Devices::Adc::IAdcController

#endif // ____x_ABI_CWindows_CDevices_CAdc_CIAdcController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                interface IAdcControllerStatics;
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics ABI::Windows::Devices::Adc::IAdcControllerStatics

#endif // ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                interface IAdcControllerStatics2;
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2 ABI::Windows::Devices::Adc::IAdcControllerStatics2

#endif // ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                class AdcController;
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("69420262-35c9-583f-a40e-c2694562c9e2"))
IAsyncOperation<ABI::Windows::Devices::Adc::AdcController*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Adc::AdcController*, ABI::Windows::Devices::Adc::IAdcController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Adc.AdcController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Adc::AdcController*> __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_t;
#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("baf66488-202f-5d51-b05e-18606c46b808"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Adc::AdcController*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Adc::AdcController*, ABI::Windows::Devices::Adc::IAdcController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Adc.AdcController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Adc::AdcController*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CAdc__CAdcController_USE
#define DEF___FIIterator_1_Windows__CDevices__CAdc__CAdcController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a10b62c1-a014-5335-8867-747fcab16005"))
IIterator<ABI::Windows::Devices::Adc::AdcController*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Adc::AdcController*, ABI::Windows::Devices::Adc::IAdcController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Adc.AdcController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Adc::AdcController*> __FIIterator_1_Windows__CDevices__CAdc__CAdcController_t;
#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CAdc__CAdcController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CAdc__CAdcController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CAdc__CAdcController_USE
#define DEF___FIIterable_1_Windows__CDevices__CAdc__CAdcController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e478aad-4861-5758-b64b-5b4f28d8f86e"))
IIterable<ABI::Windows::Devices::Adc::AdcController*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Adc::AdcController*, ABI::Windows::Devices::Adc::IAdcController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Adc.AdcController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Adc::AdcController*> __FIIterable_1_Windows__CDevices__CAdc__CAdcController_t;
#define __FIIterable_1_Windows__CDevices__CAdc__CAdcController ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CAdc__CAdcController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CAdc__CAdcController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE
#define DEF___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("27547dc1-376e-5ce4-a246-34f210c8443c"))
IVectorView<ABI::Windows::Devices::Adc::AdcController*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Adc::AdcController*, ABI::Windows::Devices::Adc::IAdcController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Adc.AdcController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Adc::AdcController*> __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_t;
#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CAdc__CAdcController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1b0cddfb-d255-5a93-bcb9-de2047a3e4f3"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CAdc__CAdcController*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CAdc__CAdcController*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Adc.AdcController>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CAdc__CAdcController*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7c4038c8-d920-53c7-a5d6-a976070d7637"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CAdc__CAdcController*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CAdc__CAdcController*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Adc.AdcController>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CAdc__CAdcController*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                namespace Provider {
                    interface IAdcProvider;
                } /* Provider */
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider ABI::Windows::Devices::Adc::Provider::IAdcProvider

#endif // ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider_FWD_DEFINED__

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
            namespace Adc {
                typedef enum AdcChannelMode : int AdcChannelMode;
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                class AdcChannel;
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Adc.AdcChannelMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                enum AdcChannelMode : int
                {
                    AdcChannelMode_SingleEnded = 0,
                    AdcChannelMode_Differential = 1,
                };
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Adc.IAdcChannel
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Adc.AdcChannel
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Adc_IAdcChannel[] = L"Windows.Devices.Adc.IAdcChannel";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                MIDL_INTERFACE("040bf414-2588-4a56-abef-73a260acc60a")
                IAdcChannel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Controller(
                        ABI::Windows::Devices::Adc::IAdcController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadValue(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadRatio(
                        DOUBLE* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdcChannel = __uuidof(IAdcChannel);
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CAdc_CIAdcChannel;
#endif /* !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Adc.IAdcController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Adc.AdcController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Adc_IAdcController[] = L"Windows.Devices.Adc.IAdcController";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                MIDL_INTERFACE("2a76e4b0-a896-4219-86b6-ea8cdce98f56")
                IAdcController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChannelCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResolutionInBits(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinValue(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxValue(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ChannelMode(
                        ABI::Windows::Devices::Adc::AdcChannelMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ChannelMode(
                        ABI::Windows::Devices::Adc::AdcChannelMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsChannelModeSupported(
                        ABI::Windows::Devices::Adc::AdcChannelMode channelMode,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenChannel(
                        INT32 channelNumber,
                        ABI::Windows::Devices::Adc::IAdcChannel** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdcController = __uuidof(IAdcController);
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CAdc_CIAdcController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Adc.IAdcControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Adc.AdcController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Adc_IAdcControllerStatics[] = L"Windows.Devices.Adc.IAdcControllerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                MIDL_INTERFACE("cce98e0c-01f8-4891-bc3b-be53ef279ca4")
                IAdcControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetControllersAsync(
                        ABI::Windows::Devices::Adc::Provider::IAdcProvider* provider,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdcControllerStatics = __uuidof(IAdcControllerStatics);
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Adc.IAdcControllerStatics2
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Adc.AdcController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Adc_IAdcControllerStatics2[] = L"Windows.Devices.Adc.IAdcControllerStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Adc {
                MIDL_INTERFACE("a2b93b1d-977b-4f5a-a5fe-a6abaffe6484")
                IAdcControllerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdcControllerStatics2 = __uuidof(IAdcControllerStatics2);
            } /* Adc */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Adc.AdcChannel
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Adc.IAdcChannel ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Adc_AdcChannel_DEFINED
#define RUNTIMECLASS_Windows_Devices_Adc_AdcChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Adc_AdcChannel[] = L"Windows.Devices.Adc.AdcChannel";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Adc.AdcController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Adc.IAdcControllerStatics interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *   Static Methods exist on the Windows.Devices.Adc.IAdcControllerStatics2 interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Adc.IAdcController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Adc_AdcController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Adc_AdcController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Adc_AdcController[] = L"Windows.Devices.Adc.AdcController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel;

#endif // ____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CIAdcController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CAdc_CIAdcController __x_ABI_CWindows_CDevices_CAdc_CIAdcController;

#endif // ____x_ABI_CWindows_CDevices_CAdc_CIAdcController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics;

#endif // ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2 __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2;

#endif // ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* This,
        __x_ABI_CWindows_CDevices_CAdc_CIAdcController** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcControllerVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController* This,
        __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CAdc__CAdcController __FIIterator_1_Windows__CDevices__CAdc__CAdcController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CAdc__CAdcController;

typedef struct __FIIterator_1_Windows__CDevices__CAdc__CAdcControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This,
        __x_ABI_CWindows_CDevices_CAdc_CIAdcController** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CAdc__CAdcController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CAdc_CIAdcController** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CAdc__CAdcControllerVtbl;

interface __FIIterator_1_Windows__CDevices__CAdc__CAdcController
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CAdc__CAdcControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CAdc__CAdcController_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CAdc__CAdcController __FIIterable_1_Windows__CDevices__CAdc__CAdcController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CAdc__CAdcController;

typedef struct __FIIterable_1_Windows__CDevices__CAdc__CAdcControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CAdc__CAdcController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CAdc__CAdcController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CAdc__CAdcController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CAdc__CAdcController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CAdc__CAdcController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CAdc__CAdcController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CAdc__CAdcController* This,
        __FIIterator_1_Windows__CDevices__CAdc__CAdcController** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CAdc__CAdcControllerVtbl;

interface __FIIterable_1_Windows__CDevices__CAdc__CAdcController
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CAdc__CAdcControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CAdc__CAdcController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CAdc__CAdcController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CAdc__CAdcController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CAdc__CAdcController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CAdc__CAdcController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CAdc__CAdcController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CAdc__CAdcController_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CAdc__CAdcController __FIVectorView_1_Windows__CDevices__CAdc__CAdcController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CAdc__CAdcController;

typedef struct __FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CAdc_CIAdcController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        __x_ABI_CWindows_CDevices_CAdc_CIAdcController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CAdc_CIAdcController** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl;

interface __FIVectorView_1_Windows__CDevices__CAdc__CAdcController
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        __FIVectorView_1_Windows__CDevices__CAdc__CAdcController** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider;

#endif // ____x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CAdc_CAdcChannelMode __x_ABI_CWindows_CDevices_CAdc_CAdcChannelMode;

/*
 *
 * Struct Windows.Devices.Adc.AdcChannelMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CAdc_CAdcChannelMode
{
    AdcChannelMode_SingleEnded = 0,
    AdcChannelMode_Differential = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Adc.IAdcChannel
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Adc.AdcChannel
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Adc_IAdcChannel[] = L"Windows.Devices.Adc.IAdcChannel";
typedef struct __x_ABI_CWindows_CDevices_CAdc_CIAdcChannelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Controller)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This,
        __x_ABI_CWindows_CDevices_CAdc_CIAdcController** value);
    HRESULT (STDMETHODCALLTYPE* ReadValue)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* ReadRatio)(__x_ABI_CWindows_CDevices_CAdc_CIAdcChannel* This,
        DOUBLE* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CAdc_CIAdcChannelVtbl;

interface __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CAdc_CIAdcChannelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_get_Controller(This, value) \
    ((This)->lpVtbl->get_Controller(This, value))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_ReadValue(This, result) \
    ((This)->lpVtbl->ReadValue(This, result))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_ReadRatio(This, result) \
    ((This)->lpVtbl->ReadRatio(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CAdc_CIAdcChannel;
#endif /* !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Adc.IAdcController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Adc.AdcController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Adc_IAdcController[] = L"Windows.Devices.Adc.IAdcController";
typedef struct __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChannelCount)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ResolutionInBits)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MinValue)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxValue)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ChannelMode)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        enum __x_ABI_CWindows_CDevices_CAdc_CAdcChannelMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ChannelMode)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        enum __x_ABI_CWindows_CDevices_CAdc_CAdcChannelMode value);
    HRESULT (STDMETHODCALLTYPE* IsChannelModeSupported)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        enum __x_ABI_CWindows_CDevices_CAdc_CAdcChannelMode channelMode,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* OpenChannel)(__x_ABI_CWindows_CDevices_CAdc_CIAdcController* This,
        INT32 channelNumber,
        __x_ABI_CWindows_CDevices_CAdc_CIAdcChannel** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerVtbl;

interface __x_ABI_CWindows_CDevices_CAdc_CIAdcController
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_get_ChannelCount(This, value) \
    ((This)->lpVtbl->get_ChannelCount(This, value))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_get_ResolutionInBits(This, value) \
    ((This)->lpVtbl->get_ResolutionInBits(This, value))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_get_MinValue(This, value) \
    ((This)->lpVtbl->get_MinValue(This, value))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_get_MaxValue(This, value) \
    ((This)->lpVtbl->get_MaxValue(This, value))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_get_ChannelMode(This, value) \
    ((This)->lpVtbl->get_ChannelMode(This, value))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_put_ChannelMode(This, value) \
    ((This)->lpVtbl->put_ChannelMode(This, value))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_IsChannelModeSupported(This, channelMode, result) \
    ((This)->lpVtbl->IsChannelModeSupported(This, channelMode, result))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcController_OpenChannel(This, channelNumber, result) \
    ((This)->lpVtbl->OpenChannel(This, channelNumber, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CAdc_CIAdcController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Adc.IAdcControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Adc.AdcController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Adc_IAdcControllerStatics[] = L"Windows.Devices.Adc.IAdcControllerStatics";
typedef struct __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetControllersAsync)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics* This,
        __x_ABI_CWindows_CDevices_CAdc_CProvider_CIAdcProvider* provider,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CAdc__CAdcController** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_GetControllersAsync(This, provider, operation) \
    ((This)->lpVtbl->GetControllersAsync(This, provider, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Adc.IAdcControllerStatics2
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Adc.AdcController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Adc_IAdcControllerStatics2[] = L"Windows.Devices.Adc.IAdcControllerStatics2";
typedef struct __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2* This,
        __FIAsyncOperation_1_Windows__CDevices__CAdc__CAdcController** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_GetDefaultAsync(This, operation) \
    ((This)->lpVtbl->GetDefaultAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CAdc_CIAdcControllerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Adc.AdcChannel
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Adc.IAdcChannel ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Adc_AdcChannel_DEFINED
#define RUNTIMECLASS_Windows_Devices_Adc_AdcChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Adc_AdcChannel[] = L"Windows.Devices.Adc.AdcChannel";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Adc.AdcController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Adc.IAdcControllerStatics interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *   Static Methods exist on the Windows.Devices.Adc.IAdcControllerStatics2 interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Adc.IAdcController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Adc_AdcController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Adc_AdcController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Adc_AdcController[] = L"Windows.Devices.Adc.AdcController";
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
#endif // __windows2Edevices2Eadc_p_h__

#endif // __windows2Edevices2Eadc_h__
