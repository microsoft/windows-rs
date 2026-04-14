
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
#ifndef __windows2Edevices2Eserialcommunication_h__
#define __windows2Edevices2Eserialcommunication_h__
#ifndef __windows2Edevices2Eserialcommunication_p_h__
#define __windows2Edevices2Eserialcommunication_p_h__


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
#include "Windows.Storage.Streams.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                interface IErrorReceivedEventArgs;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs ABI::Windows::Devices::SerialCommunication::IErrorReceivedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                interface IPinChangedEventArgs;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs ABI::Windows::Devices::SerialCommunication::IPinChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                interface ISerialDevice;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice ABI::Windows::Devices::SerialCommunication::ISerialDevice

#endif // ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                interface ISerialDeviceStatics;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics ABI::Windows::Devices::SerialCommunication::ISerialDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                class SerialDevice;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44ef26ed-c1ff-546a-a46b-6a37de9187fb"))
IAsyncOperation<ABI::Windows::Devices::SerialCommunication::SerialDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::SerialCommunication::SerialDevice*, ABI::Windows::Devices::SerialCommunication::ISerialDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.SerialCommunication.SerialDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::SerialCommunication::SerialDevice*> __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("84a34b33-06fc-5e63-8ee2-eab4ff69acb7"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::SerialCommunication::SerialDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::SerialCommunication::SerialDevice*, ABI::Windows::Devices::SerialCommunication::ISerialDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.SerialCommunication.SerialDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::SerialCommunication::SerialDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                class ErrorReceivedEventArgs;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d92ea323-b7bf-5e02-b9fb-c61f97d080e9"))
ITypedEventHandler<ABI::Windows::Devices::SerialCommunication::SerialDevice*, ABI::Windows::Devices::SerialCommunication::ErrorReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::SerialCommunication::SerialDevice*, ABI::Windows::Devices::SerialCommunication::ISerialDevice*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::SerialCommunication::ErrorReceivedEventArgs*, ABI::Windows::Devices::SerialCommunication::IErrorReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.SerialCommunication.SerialDevice, Windows.Devices.SerialCommunication.ErrorReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::SerialCommunication::SerialDevice*, ABI::Windows::Devices::SerialCommunication::ErrorReceivedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                class PinChangedEventArgs;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e496c3ef-5802-5ac4-ac2e-96bc23fa9447"))
ITypedEventHandler<ABI::Windows::Devices::SerialCommunication::SerialDevice*, ABI::Windows::Devices::SerialCommunication::PinChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::SerialCommunication::SerialDevice*, ABI::Windows::Devices::SerialCommunication::ISerialDevice*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::SerialCommunication::PinChangedEventArgs*, ABI::Windows::Devices::SerialCommunication::IPinChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.SerialCommunication.SerialDevice, Windows.Devices.SerialCommunication.PinChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::SerialCommunication::SerialDevice*, ABI::Windows::Devices::SerialCommunication::PinChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                typedef enum SerialError : int SerialError;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                typedef enum SerialHandshake : int SerialHandshake;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                typedef enum SerialParity : int SerialParity;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                typedef enum SerialPinChange : int SerialPinChange;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                typedef enum SerialStopBitCount : int SerialStopBitCount;
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                enum SerialError : int
                {
                    SerialError_Frame = 0,
                    SerialError_BufferOverrun = 1,
                    SerialError_ReceiveFull = 2,
                    SerialError_ReceiveParity = 3,
                    SerialError_TransmitFull = 4,
                };
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialHandshake
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                enum SerialHandshake : int
                {
                    SerialHandshake_None = 0,
                    SerialHandshake_RequestToSend = 1,
                    SerialHandshake_XOnXOff = 2,
                    SerialHandshake_RequestToSendXOnXOff = 3,
                };
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialParity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                enum SerialParity : int
                {
                    SerialParity_None = 0,
                    SerialParity_Odd = 1,
                    SerialParity_Even = 2,
                    SerialParity_Mark = 3,
                    SerialParity_Space = 4,
                };
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialPinChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                enum SerialPinChange : int
                {
                    SerialPinChange_BreakSignal = 0,
                    SerialPinChange_CarrierDetect = 1,
                    SerialPinChange_ClearToSend = 2,
                    SerialPinChange_DataSetReady = 3,
                    SerialPinChange_RingIndicator = 4,
                };
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialStopBitCount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                enum SerialStopBitCount : int
                {
                    SerialStopBitCount_One = 0,
                    SerialStopBitCount_OnePointFive = 1,
                    SerialStopBitCount_Two = 2,
                };
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.SerialCommunication.IErrorReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.SerialCommunication.ErrorReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_SerialCommunication_IErrorReceivedEventArgs[] = L"Windows.Devices.SerialCommunication.IErrorReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                MIDL_INTERFACE("fcc6bf59-1283-4d8a-bfdf-566b33ddb28f")
                IErrorReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Error(
                        ABI::Windows::Devices::SerialCommunication::SerialError* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IErrorReceivedEventArgs = __uuidof(IErrorReceivedEventArgs);
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.SerialCommunication.IPinChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.SerialCommunication.PinChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_SerialCommunication_IPinChangedEventArgs[] = L"Windows.Devices.SerialCommunication.IPinChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                MIDL_INTERFACE("a2bf1db0-fc9c-4607-93d0-fa5e8343ee22")
                IPinChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PinChange(
                        ABI::Windows::Devices::SerialCommunication::SerialPinChange* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPinChangedEventArgs = __uuidof(IPinChangedEventArgs);
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.SerialCommunication.ISerialDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.SerialCommunication.SerialDevice
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_SerialCommunication_ISerialDevice[] = L"Windows.Devices.SerialCommunication.ISerialDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                MIDL_INTERFACE("e187ccc6-2210-414f-b65a-f5553a03372a")
                ISerialDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BaudRate(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BaudRate(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BreakSignalState(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BreakSignalState(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BytesReceived(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CarrierDetectState(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ClearToSendState(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DataBits(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DataBits(
                        UINT16 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DataSetReadyState(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Handshake(
                        ABI::Windows::Devices::SerialCommunication::SerialHandshake* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handshake(
                        ABI::Windows::Devices::SerialCommunication::SerialHandshake value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDataTerminalReadyEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsDataTerminalReadyEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsRequestToSendEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsRequestToSendEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Parity(
                        ABI::Windows::Devices::SerialCommunication::SerialParity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Parity(
                        ABI::Windows::Devices::SerialCommunication::SerialParity value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PortName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReadTimeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReadTimeout(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StopBits(
                        ABI::Windows::Devices::SerialCommunication::SerialStopBitCount* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StopBits(
                        ABI::Windows::Devices::SerialCommunication::SerialStopBitCount value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsbVendorId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsbProductId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WriteTimeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_WriteTimeout(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputStream(
                        ABI::Windows::Storage::Streams::IInputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ErrorReceived(
                        __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs* reportHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ErrorReceived(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PinChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs* reportHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PinChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISerialDevice = __uuidof(ISerialDevice);
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.SerialCommunication.ISerialDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.SerialCommunication.SerialDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_SerialCommunication_ISerialDeviceStatics[] = L"Windows.Devices.SerialCommunication.ISerialDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace SerialCommunication {
                MIDL_INTERFACE("058c4a70-0836-4993-ae1a-b61ae3be056b")
                ISerialDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromPortName(
                        HSTRING portName,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromUsbVidPid(
                        UINT16 vendorId,
                        UINT16 productId,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISerialDeviceStatics = __uuidof(ISerialDeviceStatics);
            } /* SerialCommunication */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.SerialCommunication.ErrorReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.SerialCommunication.IErrorReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_SerialCommunication_ErrorReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_SerialCommunication_ErrorReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_SerialCommunication_ErrorReceivedEventArgs[] = L"Windows.Devices.SerialCommunication.ErrorReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.SerialCommunication.PinChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.SerialCommunication.IPinChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_SerialCommunication_PinChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_SerialCommunication_PinChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_SerialCommunication_PinChangedEventArgs[] = L"Windows.Devices.SerialCommunication.PinChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.SerialCommunication.SerialDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.SerialCommunication.ISerialDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.SerialCommunication.ISerialDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_SerialCommunication_SerialDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_SerialCommunication_SerialDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_SerialCommunication_SerialDevice[] = L"Windows.Devices.SerialCommunication.SerialDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice;

#endif // ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSerialCommunication__CSerialDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* sender,
        __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* sender,
        __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialError __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialError;

typedef enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialHandshake __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialHandshake;

typedef enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialParity __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialParity;

typedef enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialPinChange __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialPinChange;

typedef enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialStopBitCount __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialStopBitCount;

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialError
{
    SerialError_Frame = 0,
    SerialError_BufferOverrun = 1,
    SerialError_ReceiveFull = 2,
    SerialError_ReceiveParity = 3,
    SerialError_TransmitFull = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialHandshake
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialHandshake
{
    SerialHandshake_None = 0,
    SerialHandshake_RequestToSend = 1,
    SerialHandshake_XOnXOff = 2,
    SerialHandshake_RequestToSendXOnXOff = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialParity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialParity
{
    SerialParity_None = 0,
    SerialParity_Odd = 1,
    SerialParity_Even = 2,
    SerialParity_Mark = 3,
    SerialParity_Space = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialPinChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialPinChange
{
    SerialPinChange_BreakSignal = 0,
    SerialPinChange_CarrierDetect = 1,
    SerialPinChange_ClearToSend = 2,
    SerialPinChange_DataSetReady = 3,
    SerialPinChange_RingIndicator = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.SerialCommunication.SerialStopBitCount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialStopBitCount
{
    SerialStopBitCount_One = 0,
    SerialStopBitCount_OnePointFive = 1,
    SerialStopBitCount_Two = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.SerialCommunication.IErrorReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.SerialCommunication.ErrorReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_SerialCommunication_IErrorReceivedEventArgs[] = L"Windows.Devices.SerialCommunication.IErrorReceivedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialError* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CIErrorReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.SerialCommunication.IPinChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.SerialCommunication.PinChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_SerialCommunication_IPinChangedEventArgs[] = L"Windows.Devices.SerialCommunication.IPinChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PinChange)(__x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialPinChange* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_get_PinChange(This, value) \
    ((This)->lpVtbl->get_PinChange(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CIPinChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.SerialCommunication.ISerialDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.SerialCommunication.SerialDevice
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_SerialCommunication_ISerialDevice[] = L"Windows.Devices.SerialCommunication.ISerialDevice";
typedef struct __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BaudRate)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_BaudRate)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BreakSignalState)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_BreakSignalState)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_BytesReceived)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CarrierDetectState)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ClearToSendState)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DataBits)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* put_DataBits)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        UINT16 value);
    HRESULT (STDMETHODCALLTYPE* get_DataSetReadyState)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Handshake)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialHandshake* value);
    HRESULT (STDMETHODCALLTYPE* put_Handshake)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialHandshake value);
    HRESULT (STDMETHODCALLTYPE* get_IsDataTerminalReadyEnabled)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsDataTerminalReadyEnabled)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsRequestToSendEnabled)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsRequestToSendEnabled)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Parity)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialParity* value);
    HRESULT (STDMETHODCALLTYPE* put_Parity)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialParity value);
    HRESULT (STDMETHODCALLTYPE* get_PortName)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ReadTimeout)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_ReadTimeout)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_StopBits)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialStopBitCount* value);
    HRESULT (STDMETHODCALLTYPE* put_StopBits)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        enum __x_ABI_CWindows_CDevices_CSerialCommunication_CSerialStopBitCount value);
    HRESULT (STDMETHODCALLTYPE* get_UsbVendorId)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_UsbProductId)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_WriteTimeout)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_WriteTimeout)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_InputStream)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* add_ErrorReceived)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CErrorReceivedEventArgs* reportHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ErrorReceived)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PinChanged)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CSerialCommunication__CSerialDevice_Windows__CDevices__CSerialCommunication__CPinChangedEventArgs* reportHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PinChanged)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_BaudRate(This, value) \
    ((This)->lpVtbl->get_BaudRate(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_BaudRate(This, value) \
    ((This)->lpVtbl->put_BaudRate(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_BreakSignalState(This, value) \
    ((This)->lpVtbl->get_BreakSignalState(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_BreakSignalState(This, value) \
    ((This)->lpVtbl->put_BreakSignalState(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_BytesReceived(This, value) \
    ((This)->lpVtbl->get_BytesReceived(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_CarrierDetectState(This, value) \
    ((This)->lpVtbl->get_CarrierDetectState(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_ClearToSendState(This, value) \
    ((This)->lpVtbl->get_ClearToSendState(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_DataBits(This, value) \
    ((This)->lpVtbl->get_DataBits(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_DataBits(This, value) \
    ((This)->lpVtbl->put_DataBits(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_DataSetReadyState(This, value) \
    ((This)->lpVtbl->get_DataSetReadyState(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_Handshake(This, value) \
    ((This)->lpVtbl->get_Handshake(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_Handshake(This, value) \
    ((This)->lpVtbl->put_Handshake(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_IsDataTerminalReadyEnabled(This, value) \
    ((This)->lpVtbl->get_IsDataTerminalReadyEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_IsDataTerminalReadyEnabled(This, value) \
    ((This)->lpVtbl->put_IsDataTerminalReadyEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_IsRequestToSendEnabled(This, value) \
    ((This)->lpVtbl->get_IsRequestToSendEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_IsRequestToSendEnabled(This, value) \
    ((This)->lpVtbl->put_IsRequestToSendEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_Parity(This, value) \
    ((This)->lpVtbl->get_Parity(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_Parity(This, value) \
    ((This)->lpVtbl->put_Parity(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_PortName(This, value) \
    ((This)->lpVtbl->get_PortName(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_ReadTimeout(This, value) \
    ((This)->lpVtbl->get_ReadTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_ReadTimeout(This, value) \
    ((This)->lpVtbl->put_ReadTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_StopBits(This, value) \
    ((This)->lpVtbl->get_StopBits(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_StopBits(This, value) \
    ((This)->lpVtbl->put_StopBits(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_UsbVendorId(This, value) \
    ((This)->lpVtbl->get_UsbVendorId(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_UsbProductId(This, value) \
    ((This)->lpVtbl->get_UsbProductId(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_WriteTimeout(This, value) \
    ((This)->lpVtbl->get_WriteTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_put_WriteTimeout(This, value) \
    ((This)->lpVtbl->put_WriteTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_InputStream(This, value) \
    ((This)->lpVtbl->get_InputStream(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_add_ErrorReceived(This, reportHandler, token) \
    ((This)->lpVtbl->add_ErrorReceived(This, reportHandler, token))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_remove_ErrorReceived(This, token) \
    ((This)->lpVtbl->remove_ErrorReceived(This, token))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_add_PinChanged(This, reportHandler, token) \
    ((This)->lpVtbl->add_PinChanged(This, reportHandler, token))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_remove_PinChanged(This, token) \
    ((This)->lpVtbl->remove_PinChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.SerialCommunication.ISerialDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.SerialCommunication.SerialDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_SerialCommunication_ISerialDeviceStatics[] = L"Windows.Devices.SerialCommunication.ISerialDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromPortName)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This,
        HSTRING portName,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromUsbVidPid)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This,
        UINT16 vendorId,
        UINT16 productId,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CSerialCommunication__CSerialDevice** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_GetDeviceSelector(This, value) \
    ((This)->lpVtbl->GetDeviceSelector(This, value))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_GetDeviceSelectorFromPortName(This, portName, result) \
    ((This)->lpVtbl->GetDeviceSelectorFromPortName(This, portName, result))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_GetDeviceSelectorFromUsbVidPid(This, vendorId, productId, result) \
    ((This)->lpVtbl->GetDeviceSelectorFromUsbVidPid(This, vendorId, productId, result))

#define __x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_FromIdAsync(This, deviceId, result) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSerialCommunication_CISerialDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.SerialCommunication.ErrorReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.SerialCommunication.IErrorReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_SerialCommunication_ErrorReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_SerialCommunication_ErrorReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_SerialCommunication_ErrorReceivedEventArgs[] = L"Windows.Devices.SerialCommunication.ErrorReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.SerialCommunication.PinChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.SerialCommunication.IPinChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_SerialCommunication_PinChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_SerialCommunication_PinChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_SerialCommunication_PinChangedEventArgs[] = L"Windows.Devices.SerialCommunication.PinChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.SerialCommunication.SerialDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.SerialCommunication.ISerialDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.SerialCommunication.ISerialDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_SerialCommunication_SerialDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_SerialCommunication_SerialDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_SerialCommunication_SerialDevice[] = L"Windows.Devices.SerialCommunication.SerialDevice";
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
#endif // __windows2Edevices2Eserialcommunication_p_h__

#endif // __windows2Edevices2Eserialcommunication_h__
