
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
#ifndef __windows2Edevices2Ei2c2Eprovider_h__
#define __windows2Edevices2Ei2c2Eprovider_h__
#ifndef __windows2Edevices2Ei2c2Eprovider_p_h__
#define __windows2Edevices2Ei2c2Eprovider_p_h__


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
#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    interface II2cControllerProvider;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider ABI::Windows::Devices::I2c::Provider::II2cControllerProvider

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    interface II2cDeviceProvider;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider ABI::Windows::Devices::I2c::Provider::II2cDeviceProvider

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    interface II2cProvider;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider ABI::Windows::Devices::I2c::Provider::II2cProvider

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    interface IProviderI2cConnectionSettings;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings ABI::Windows::Devices::I2c::Provider::IProviderI2cConnectionSettings

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#define DEF___FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6232961-c660-50a1-82e8-12892fcd91f7"))
IIterator<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*> : IIterator_impl<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.I2c.Provider.II2cControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*> __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t;
#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#define DEF___FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("11341a6c-3a02-5f73-9db8-c3ec5823e35d"))
IIterable<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*> : IIterable_impl<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.I2c.Provider.II2cControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*> __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t;
#define __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#define DEF___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("511f8a39-98ca-550d-af25-1df2c1193c01"))
IVectorView<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*> : IVectorView_impl<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.I2c.Provider.II2cControllerProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::I2c::Provider::II2cControllerProvider*> __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t;
#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5fe77838-1125-5b2c-a281-e06a3dfbb76e"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.I2c.Provider.II2cControllerProvider>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("771e22ed-da9e-50be-b730-a3bada6bfb25"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.I2c.Provider.II2cControllerProvider>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_USE */

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
            namespace I2c {
                namespace Provider {
                    typedef enum ProviderI2cBusSpeed : int ProviderI2cBusSpeed;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    typedef enum ProviderI2cSharingMode : int ProviderI2cSharingMode;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    typedef enum ProviderI2cTransferStatus : int ProviderI2cTransferStatus;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    typedef struct ProviderI2cTransferResult ProviderI2cTransferResult;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    class ProviderI2cConnectionSettings;
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.I2c.Provider.ProviderI2cBusSpeed
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    enum ProviderI2cBusSpeed : int
                    {
                        ProviderI2cBusSpeed_StandardMode = 0,
                        ProviderI2cBusSpeed_FastMode = 1,
                    };
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.I2c.Provider.ProviderI2cSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    enum ProviderI2cSharingMode : int
                    {
                        ProviderI2cSharingMode_Exclusive = 0,
                        ProviderI2cSharingMode_Shared = 1,
                    };
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.I2c.Provider.ProviderI2cTransferStatus
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    enum ProviderI2cTransferStatus : int
                    {
                        ProviderI2cTransferStatus_FullTransfer = 0,
                        ProviderI2cTransferStatus_PartialTransfer = 1,
                        ProviderI2cTransferStatus_SlaveAddressNotAcknowledged = 2,
                    };
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.I2c.Provider.ProviderI2cTransferResult
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    struct ProviderI2cTransferResult
                    {
                        ABI::Windows::Devices::I2c::Provider::ProviderI2cTransferStatus Status;
                        UINT32 BytesTransferred;
                    };
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.I2c.Provider.II2cControllerProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_I2c_Provider_II2cControllerProvider[] = L"Windows.Devices.I2c.Provider.II2cControllerProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    MIDL_INTERFACE("61c2bb82-4510-4163-a87c-4e15a9558980")
                    II2cControllerProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceProvider(
                            ABI::Windows::Devices::I2c::Provider::IProviderI2cConnectionSettings* settings,
                            ABI::Windows::Devices::I2c::Provider::II2cDeviceProvider** device
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_II2cControllerProvider = __uuidof(II2cControllerProvider);
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.I2c.Provider.II2cDeviceProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_I2c_Provider_II2cDeviceProvider[] = L"Windows.Devices.I2c.Provider.II2cDeviceProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    MIDL_INTERFACE("ad342654-57e8-453e-8329-d1e447d103a9")
                    II2cDeviceProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Write(
                            UINT32 bufferLength,
                            BYTE* buffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE WritePartial(
                            UINT32 bufferLength,
                            BYTE* buffer,
                            ABI::Windows::Devices::I2c::Provider::ProviderI2cTransferResult* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Read(
                            UINT32 bufferLength,
                            BYTE* buffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReadPartial(
                            UINT32 bufferLength,
                            BYTE* buffer,
                            ABI::Windows::Devices::I2c::Provider::ProviderI2cTransferResult* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE WriteRead(
                            UINT32 writeBufferLength,
                            BYTE* writeBuffer,
                            UINT32 readBufferLength,
                            BYTE* readBuffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE WriteReadPartial(
                            UINT32 writeBufferLength,
                            BYTE* writeBuffer,
                            UINT32 readBufferLength,
                            BYTE* readBuffer,
                            ABI::Windows::Devices::I2c::Provider::ProviderI2cTransferResult* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_II2cDeviceProvider = __uuidof(II2cDeviceProvider);
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.I2c.Provider.II2cProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_I2c_Provider_II2cProvider[] = L"Windows.Devices.I2c.Provider.II2cProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    MIDL_INTERFACE("6f13083e-bf62-4fe2-a95a-f08999669818")
                    II2cProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetControllersAsync(
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_II2cProvider = __uuidof(II2cProvider);
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.I2c.Provider.IProviderI2cConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_I2c_Provider_IProviderI2cConnectionSettings[] = L"Windows.Devices.I2c.Provider.IProviderI2cConnectionSettings";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace I2c {
                namespace Provider {
                    MIDL_INTERFACE("e9db4e34-e510-44b7-809d-f2f85b555339")
                    IProviderI2cConnectionSettings : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SlaveAddress(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SlaveAddress(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BusSpeed(
                            ABI::Windows::Devices::I2c::Provider::ProviderI2cBusSpeed* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_BusSpeed(
                            ABI::Windows::Devices::I2c::Provider::ProviderI2cBusSpeed value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SharingMode(
                            ABI::Windows::Devices::I2c::Provider::ProviderI2cSharingMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SharingMode(
                            ABI::Windows::Devices::I2c::Provider::ProviderI2cSharingMode value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IProviderI2cConnectionSettings = __uuidof(IProviderI2cConnectionSettings);
                } /* Provider */
            } /* I2c */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.I2c.Provider.IProviderI2cConnectionSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_I2c_Provider_ProviderI2cConnectionSettings_DEFINED
#define RUNTIMECLASS_Windows_Devices_I2c_Provider_ProviderI2cConnectionSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_I2c_Provider_ProviderI2cConnectionSettings[] = L"Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider;

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider;

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider;

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings;

#endif // ____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

typedef struct __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl;

interface __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

typedef struct __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        __FIIterator_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl;

interface __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

typedef struct __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl;

interface __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        __FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cBusSpeed __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cBusSpeed;

typedef enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cSharingMode __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cSharingMode;

typedef enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferStatus __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferStatus;

typedef struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferResult __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferResult;

/*
 *
 * Struct Windows.Devices.I2c.Provider.ProviderI2cBusSpeed
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cBusSpeed
{
    ProviderI2cBusSpeed_StandardMode = 0,
    ProviderI2cBusSpeed_FastMode = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.I2c.Provider.ProviderI2cSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cSharingMode
{
    ProviderI2cSharingMode_Exclusive = 0,
    ProviderI2cSharingMode_Shared = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.I2c.Provider.ProviderI2cTransferStatus
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferStatus
{
    ProviderI2cTransferStatus_FullTransfer = 0,
    ProviderI2cTransferStatus_PartialTransfer = 1,
    ProviderI2cTransferStatus_SlaveAddressNotAcknowledged = 2,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Devices.I2c.Provider.ProviderI2cTransferResult
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferResult
{
    enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferStatus Status;
    UINT32 BytesTransferred;
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.I2c.Provider.II2cControllerProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_I2c_Provider_II2cControllerProvider[] = L"Windows.Devices.I2c.Provider.II2cControllerProvider";
typedef struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceProvider)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider* This,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* settings,
        __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider** device);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProviderVtbl;

interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_GetDeviceProvider(This, settings, device) \
    ((This)->lpVtbl->GetDeviceProvider(This, settings, device))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.I2c.Provider.II2cDeviceProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_I2c_Provider_II2cDeviceProvider[] = L"Windows.Devices.I2c.Provider.II2cDeviceProvider";
typedef struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* Write)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        UINT32 bufferLength,
        BYTE* buffer);
    HRESULT (STDMETHODCALLTYPE* WritePartial)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        UINT32 bufferLength,
        BYTE* buffer,
        struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferResult* result);
    HRESULT (STDMETHODCALLTYPE* Read)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        UINT32 bufferLength,
        BYTE* buffer);
    HRESULT (STDMETHODCALLTYPE* ReadPartial)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        UINT32 bufferLength,
        BYTE* buffer,
        struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferResult* result);
    HRESULT (STDMETHODCALLTYPE* WriteRead)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        UINT32 writeBufferLength,
        BYTE* writeBuffer,
        UINT32 readBufferLength,
        BYTE* readBuffer);
    HRESULT (STDMETHODCALLTYPE* WriteReadPartial)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider* This,
        UINT32 writeBufferLength,
        BYTE* writeBuffer,
        UINT32 readBufferLength,
        BYTE* readBuffer,
        struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cTransferResult* result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProviderVtbl;

interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_Write(This, bufferLength, buffer) \
    ((This)->lpVtbl->Write(This, bufferLength, buffer))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_WritePartial(This, bufferLength, buffer, result) \
    ((This)->lpVtbl->WritePartial(This, bufferLength, buffer, result))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_Read(This, bufferLength, buffer) \
    ((This)->lpVtbl->Read(This, bufferLength, buffer))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_ReadPartial(This, bufferLength, buffer, result) \
    ((This)->lpVtbl->ReadPartial(This, bufferLength, buffer, result))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_WriteRead(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer) \
    ((This)->lpVtbl->WriteRead(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_WriteReadPartial(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer, result) \
    ((This)->lpVtbl->WriteReadPartial(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cDeviceProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.I2c.Provider.II2cProvider
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_I2c_Provider_II2cProvider[] = L"Windows.Devices.I2c.Provider.II2cProvider";
typedef struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetControllersAsync)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CI2c__CProvider__CII2cControllerProvider** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProviderVtbl;

interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_GetControllersAsync(This, operation) \
    ((This)->lpVtbl->GetControllersAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CII2cProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.I2c.Provider.IProviderI2cConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_I2c_Provider_IProviderI2cConnectionSettings[] = L"Windows.Devices.I2c.Provider.IProviderI2cConnectionSettings";
typedef struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SlaveAddress)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_SlaveAddress)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BusSpeed)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cBusSpeed* value);
    HRESULT (STDMETHODCALLTYPE* put_BusSpeed)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cBusSpeed value);
    HRESULT (STDMETHODCALLTYPE* get_SharingMode)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cSharingMode* value);
    HRESULT (STDMETHODCALLTYPE* put_SharingMode)(__x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CI2c_CProvider_CProviderI2cSharingMode value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettingsVtbl;

interface __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_get_SlaveAddress(This, value) \
    ((This)->lpVtbl->get_SlaveAddress(This, value))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_put_SlaveAddress(This, value) \
    ((This)->lpVtbl->put_SlaveAddress(This, value))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_get_BusSpeed(This, value) \
    ((This)->lpVtbl->get_BusSpeed(This, value))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_put_BusSpeed(This, value) \
    ((This)->lpVtbl->put_BusSpeed(This, value))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_get_SharingMode(This, value) \
    ((This)->lpVtbl->get_SharingMode(This, value))

#define __x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_put_SharingMode(This, value) \
    ((This)->lpVtbl->put_SharingMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CI2c_CProvider_CIProviderI2cConnectionSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.I2c.Provider.IProviderI2cConnectionSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_I2c_Provider_ProviderI2cConnectionSettings_DEFINED
#define RUNTIMECLASS_Windows_Devices_I2c_Provider_ProviderI2cConnectionSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_I2c_Provider_ProviderI2cConnectionSettings[] = L"Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings";
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
#endif // __windows2Edevices2Ei2c2Eprovider_p_h__

#endif // __windows2Edevices2Ei2c2Eprovider_h__
