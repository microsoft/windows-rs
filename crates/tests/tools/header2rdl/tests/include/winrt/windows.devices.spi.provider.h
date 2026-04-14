
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
#ifndef __windows2Edevices2Espi2Eprovider_h__
#define __windows2Edevices2Espi2Eprovider_h__
#ifndef __windows2Edevices2Espi2Eprovider_p_h__
#define __windows2Edevices2Espi2Eprovider_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    interface IProviderSpiConnectionSettings;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings ABI::Windows::Devices::Spi::Provider::IProviderSpiConnectionSettings

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    interface IProviderSpiConnectionSettingsFactory;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory ABI::Windows::Devices::Spi::Provider::IProviderSpiConnectionSettingsFactory

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    interface ISpiControllerProvider;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    interface ISpiDeviceProvider;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider ABI::Windows::Devices::Spi::Provider::ISpiDeviceProvider

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    interface ISpiProvider;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider ABI::Windows::Devices::Spi::Provider::ISpiProvider

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#define DEF___FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cf1d15d3-a6c8-56dd-80c8-e8d960262277"))
IIterator<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*> : IIterator_impl<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Spi.Provider.ISpiControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*> __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t;
#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#define DEF___FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("71ba027d-8c84-58b1-8d66-9177c11698eb"))
IIterable<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*> : IIterable_impl<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Spi.Provider.ISpiControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*> __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t;
#define __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#define DEF___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("96a4919b-3229-5e41-8b10-c8198c44f10c"))
IVectorView<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*> : IVectorView_impl<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Spi.Provider.ISpiControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Spi::Provider::ISpiControllerProvider*> __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t;
#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b3af3490-dede-59d1-b562-1f6be71ae139"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Spi.Provider.ISpiControllerProvider>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e9e2ae03-42d6-5211-ab52-325e722e2611"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Spi.Provider.ISpiControllerProvider>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

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
            namespace Spi {
                namespace Provider {
                    typedef enum ProviderSpiMode : int ProviderSpiMode;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    typedef enum ProviderSpiSharingMode : int ProviderSpiSharingMode;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    class ProviderSpiConnectionSettings;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Spi.Provider.ProviderSpiMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    enum ProviderSpiMode : int
                    {
                        ProviderSpiMode_Mode0 = 0,
                        ProviderSpiMode_Mode1 = 1,
                        ProviderSpiMode_Mode2 = 2,
                        ProviderSpiMode_Mode3 = 3,
                    };
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Spi.Provider.ProviderSpiSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    enum ProviderSpiSharingMode : int
                    {
                        ProviderSpiSharingMode_Exclusive = 0,
                        ProviderSpiSharingMode_Shared = 1,
                    };
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.IProviderSpiConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_IProviderSpiConnectionSettings[] = L"Windows.Devices.Spi.Provider.IProviderSpiConnectionSettings";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    MIDL_INTERFACE("f6034550-a542-4ec0-9601-a4dd68f8697b")
                    IProviderSpiConnectionSettings : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ChipSelectLine(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ChipSelectLine(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Mode(
                            ABI::Windows::Devices::Spi::Provider::ProviderSpiMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Mode(
                            ABI::Windows::Devices::Spi::Provider::ProviderSpiMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DataBitLength(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DataBitLength(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ClockFrequency(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ClockFrequency(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SharingMode(
                            ABI::Windows::Devices::Spi::Provider::ProviderSpiSharingMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SharingMode(
                            ABI::Windows::Devices::Spi::Provider::ProviderSpiSharingMode value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IProviderSpiConnectionSettings = __uuidof(IProviderSpiConnectionSettings);
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.IProviderSpiConnectionSettingsFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_IProviderSpiConnectionSettingsFactory[] = L"Windows.Devices.Spi.Provider.IProviderSpiConnectionSettingsFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    MIDL_INTERFACE("66456b5a-0c79-43e3-9f3c-e59780ac18fa")
                    IProviderSpiConnectionSettingsFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            INT32 chipSelectLine,
                            ABI::Windows::Devices::Spi::Provider::IProviderSpiConnectionSettings** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IProviderSpiConnectionSettingsFactory = __uuidof(IProviderSpiConnectionSettingsFactory);
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.ISpiControllerProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_ISpiControllerProvider[] = L"Windows.Devices.Spi.Provider.ISpiControllerProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    MIDL_INTERFACE("c1686504-02ce-4226-a385-4f11fb04b41b")
                    ISpiControllerProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceProvider(
                            ABI::Windows::Devices::Spi::Provider::IProviderSpiConnectionSettings* settings,
                            ABI::Windows::Devices::Spi::Provider::ISpiDeviceProvider** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpiControllerProvider = __uuidof(ISpiControllerProvider);
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.ISpiDeviceProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_ISpiDeviceProvider[] = L"Windows.Devices.Spi.Provider.ISpiDeviceProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    MIDL_INTERFACE("0d1c3443-304b-405c-b4f7-f5ab1074461e")
                    ISpiDeviceProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ConnectionSettings(
                            ABI::Windows::Devices::Spi::Provider::IProviderSpiConnectionSettings** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Write(
                            UINT32 bufferLength,
                            BYTE* buffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Read(
                            UINT32 bufferLength,
                            BYTE* buffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TransferSequential(
                            UINT32 writeBufferLength,
                            BYTE* writeBuffer,
                            UINT32 readBufferLength,
                            BYTE* readBuffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TransferFullDuplex(
                            UINT32 writeBufferLength,
                            BYTE* writeBuffer,
                            UINT32 readBufferLength,
                            BYTE* readBuffer
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpiDeviceProvider = __uuidof(ISpiDeviceProvider);
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.ISpiProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_ISpiProvider[] = L"Windows.Devices.Spi.Provider.ISpiProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    MIDL_INTERFACE("96b461e2-77d4-48ce-aaa0-75715a8362cf")
                    ISpiProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetControllersAsync(
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpiProvider = __uuidof(ISpiProvider);
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Spi.Provider.IProviderSpiConnectionSettingsFactory interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.Provider.IProviderSpiConnectionSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_Provider_ProviderSpiConnectionSettings_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_Provider_ProviderSpiConnectionSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_Provider_ProviderSpiConnectionSettings[] = L"Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

typedef struct __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl;

interface __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

typedef struct __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        __FIIterator_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl;

interface __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

typedef struct __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl;

interface __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        __FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiMode __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiMode;

typedef enum __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiSharingMode __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiSharingMode;

/*
 *
 * Struct Windows.Devices.Spi.Provider.ProviderSpiMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiMode
{
    ProviderSpiMode_Mode0 = 0,
    ProviderSpiMode_Mode1 = 1,
    ProviderSpiMode_Mode2 = 2,
    ProviderSpiMode_Mode3 = 3,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.Spi.Provider.ProviderSpiSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiSharingMode
{
    ProviderSpiSharingMode_Exclusive = 0,
    ProviderSpiSharingMode_Shared = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.IProviderSpiConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_IProviderSpiConnectionSettings[] = L"Windows.Devices.Spi.Provider.IProviderSpiConnectionSettings";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChipSelectLine)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ChipSelectLine)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiMode value);
    HRESULT (STDMETHODCALLTYPE* get_DataBitLength)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DataBitLength)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ClockFrequency)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ClockFrequency)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_SharingMode)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiSharingMode* value);
    HRESULT (STDMETHODCALLTYPE* put_SharingMode)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CSpi_CProvider_CProviderSpiSharingMode value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_get_ChipSelectLine(This, value) \
    ((This)->lpVtbl->get_ChipSelectLine(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_put_ChipSelectLine(This, value) \
    ((This)->lpVtbl->put_ChipSelectLine(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_get_DataBitLength(This, value) \
    ((This)->lpVtbl->get_DataBitLength(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_put_DataBitLength(This, value) \
    ((This)->lpVtbl->put_DataBitLength(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_get_ClockFrequency(This, value) \
    ((This)->lpVtbl->get_ClockFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_put_ClockFrequency(This, value) \
    ((This)->lpVtbl->put_ClockFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_get_SharingMode(This, value) \
    ((This)->lpVtbl->get_SharingMode(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_put_SharingMode(This, value) \
    ((This)->lpVtbl->put_SharingMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.IProviderSpiConnectionSettingsFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_IProviderSpiConnectionSettingsFactory[] = L"Windows.Devices.Spi.Provider.IProviderSpiConnectionSettingsFactory";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory* This,
        INT32 chipSelectLine,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_Create(This, chipSelectLine, value) \
    ((This)->lpVtbl->Create(This, chipSelectLine, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettingsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.ISpiControllerProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_ISpiControllerProvider[] = L"Windows.Devices.Spi.Provider.ISpiControllerProvider";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceProvider)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider* This,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings* settings,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProviderVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_GetDeviceProvider(This, settings, result) \
    ((This)->lpVtbl->GetDeviceProvider(This, settings, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.ISpiDeviceProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_ISpiDeviceProvider[] = L"Windows.Devices.Spi.Provider.ISpiDeviceProvider";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionSettings)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CIProviderSpiConnectionSettings** value);
    HRESULT (STDMETHODCALLTYPE* Write)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        UINT32 bufferLength,
        BYTE* buffer);
    HRESULT (STDMETHODCALLTYPE* Read)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        UINT32 bufferLength,
        BYTE* buffer);
    HRESULT (STDMETHODCALLTYPE* TransferSequential)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        UINT32 writeBufferLength,
        BYTE* writeBuffer,
        UINT32 readBufferLength,
        BYTE* readBuffer);
    HRESULT (STDMETHODCALLTYPE* TransferFullDuplex)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider* This,
        UINT32 writeBufferLength,
        BYTE* writeBuffer,
        UINT32 readBufferLength,
        BYTE* readBuffer);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProviderVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_get_ConnectionSettings(This, value) \
    ((This)->lpVtbl->get_ConnectionSettings(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_Write(This, bufferLength, buffer) \
    ((This)->lpVtbl->Write(This, bufferLength, buffer))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_Read(This, bufferLength, buffer) \
    ((This)->lpVtbl->Read(This, bufferLength, buffer))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_TransferSequential(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer) \
    ((This)->lpVtbl->TransferSequential(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_TransferFullDuplex(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer) \
    ((This)->lpVtbl->TransferFullDuplex(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiDeviceProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.Provider.ISpiProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_Provider_ISpiProvider[] = L"Windows.Devices.Spi.Provider.ISpiProvider";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetControllersAsync)(__x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CProvider__CISpiControllerProvider** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProviderVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_GetControllersAsync(This, result) \
    ((This)->lpVtbl->GetControllersAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Spi.Provider.IProviderSpiConnectionSettingsFactory interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.Provider.IProviderSpiConnectionSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_Provider_ProviderSpiConnectionSettings_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_Provider_ProviderSpiConnectionSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_Provider_ProviderSpiConnectionSettings[] = L"Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Espi2Eprovider_p_h__

#endif // __windows2Edevices2Espi2Eprovider_h__
