
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
#ifndef __windows2Esystem2Eremotedesktop2Einput_h__
#define __windows2Esystem2Eremotedesktop2Einput_h__
#ifndef __windows2Esystem2Eremotedesktop2Einput_p_h__
#define __windows2Esystem2Eremotedesktop2Einput_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    interface IRemoteTextConnectionDataHandler;
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnectionDataHandler

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    interface IRemoteTextConnection;
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnection

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    interface IRemoteTextConnection2;
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2 ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnection2

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    interface IRemoteTextConnectionFactory;
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnectionFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    interface IRemoteTextConnectionFactory2;
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2 ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnectionFactory2

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    typedef enum RemoteKeyEventAttributes : unsigned int RemoteKeyEventAttributes;
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    typedef enum RemoteTextConnectionOptions : unsigned int RemoteTextConnectionOptions;
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    class RemoteTextConnection;
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.RemoteDesktop.Input.RemoteKeyEventAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    enum RemoteKeyEventAttributes : unsigned int
                    {
                        RemoteKeyEventAttributes_None = 0,
                        RemoteKeyEventAttributes_KeyUp = 0x1,
                        RemoteKeyEventAttributes_Repeat = 0x2,
                        RemoteKeyEventAttributes_Extended = 0x4,
                        RemoteKeyEventAttributes_Extended1 = 0x8,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(RemoteKeyEventAttributes)
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.System.RemoteDesktop.Input.RemoteTextConnectionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    enum RemoteTextConnectionOptions : unsigned int
                    {
                        RemoteTextConnectionOptions_None = 0,
                        RemoteTextConnectionOptions_EnablePredictedKeyReporting = 0x1,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(RemoteTextConnectionOptions)
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Delegate Windows.System.RemoteDesktop.Input.RemoteTextConnectionDataHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    MIDL_INTERFACE("099ffbc8-8bcb-41b5-b056-57e77021bf1b")
                    IRemoteTextConnectionDataHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            UINT32 pduDataLength,
                            BYTE* pduData,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteTextConnectionDataHandler = __uuidof(IRemoteTextConnectionDataHandler);
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Input.IRemoteTextConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Input_IRemoteTextConnection[] = L"Windows.System.RemoteDesktop.Input.IRemoteTextConnection";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    MIDL_INTERFACE("4e7bb02a-183e-5e66-b5e4-3e6e5c570cf1")
                    IRemoteTextConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RegisterThread(
                            UINT32 threadId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnregisterThread(
                            UINT32 threadId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportDataReceived(
                            UINT32 pduDataLength,
                            BYTE* pduData
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteTextConnection = __uuidof(IRemoteTextConnection);
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Input.IRemoteTextConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Input_IRemoteTextConnection2[] = L"Windows.System.RemoteDesktop.Input.IRemoteTextConnection2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    MIDL_INTERFACE("05f99345-84c8-56c5-934f-73ea00f8c2d5")
                    IRemoteTextConnection2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportPredictedKeyEvent(
                            UINT16 scanCode,
                            ABI::Windows::System::RemoteDesktop::Input::RemoteKeyEventAttributes attributes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteTextConnection2 = __uuidof(IRemoteTextConnection2);
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Input_IRemoteTextConnectionFactory[] = L"Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    MIDL_INTERFACE("88e075c2-0cae-596c-850f-78d345cd728b")
                    IRemoteTextConnectionFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            GUID connectionId,
                            ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnectionDataHandler* pduForwarder,
                            ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteTextConnectionFactory = __uuidof(IRemoteTextConnectionFactory);
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Input_IRemoteTextConnectionFactory2[] = L"Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Input {
                    MIDL_INTERFACE("9425c7d9-ed9b-5d00-99cc-b0b8dc9e4c60")
                    IRemoteTextConnectionFactory2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            GUID connectionId,
                            ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnectionDataHandler* pduForwarder,
                            ABI::Windows::System::RemoteDesktop::Input::RemoteTextConnectionOptions options,
                            ABI::Windows::System::RemoteDesktop::Input::IRemoteTextConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteTextConnectionFactory2 = __uuidof(IRemoteTextConnectionFactory2);
                } /* Input */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory2 interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Input.IRemoteTextConnection ** Default Interface **
 *    Windows.System.RemoteDesktop.Input.IRemoteTextConnection2
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Input_RemoteTextConnection_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Input_RemoteTextConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Input_RemoteTextConnection[] = L"Windows.System.RemoteDesktop.Input.RemoteTextConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2 __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2 __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CRemoteKeyEventAttributes __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CRemoteKeyEventAttributes;

typedef enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CRemoteTextConnectionOptions __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CRemoteTextConnectionOptions;

/*
 *
 * Struct Windows.System.RemoteDesktop.Input.RemoteKeyEventAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CRemoteKeyEventAttributes
{
    RemoteKeyEventAttributes_None = 0,
    RemoteKeyEventAttributes_KeyUp = 0x1,
    RemoteKeyEventAttributes_Repeat = 0x2,
    RemoteKeyEventAttributes_Extended = 0x4,
    RemoteKeyEventAttributes_Extended1 = 0x8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.System.RemoteDesktop.Input.RemoteTextConnectionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CRemoteTextConnectionOptions
{
    RemoteTextConnectionOptions_None = 0,
    RemoteTextConnectionOptions_EnablePredictedKeyReporting = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Delegate Windows.System.RemoteDesktop.Input.RemoteTextConnectionDataHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler* This,
        UINT32 pduDataLength,
        BYTE* pduData,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandlerVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_Invoke(This, pduDataLength, pduData, result) \
    ((This)->lpVtbl->Invoke(This, pduDataLength, pduData, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Input.IRemoteTextConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Input_IRemoteTextConnection[] = L"Windows.System.RemoteDesktop.Input.IRemoteTextConnection";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* RegisterThread)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        UINT32 threadId);
    HRESULT (STDMETHODCALLTYPE* UnregisterThread)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        UINT32 threadId);
    HRESULT (STDMETHODCALLTYPE* ReportDataReceived)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection* This,
        UINT32 pduDataLength,
        BYTE* pduData);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_RegisterThread(This, threadId) \
    ((This)->lpVtbl->RegisterThread(This, threadId))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_UnregisterThread(This, threadId) \
    ((This)->lpVtbl->UnregisterThread(This, threadId))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_ReportDataReceived(This, pduDataLength, pduData) \
    ((This)->lpVtbl->ReportDataReceived(This, pduDataLength, pduData))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Input.IRemoteTextConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Input_IRemoteTextConnection2[] = L"Windows.System.RemoteDesktop.Input.IRemoteTextConnection2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportPredictedKeyEvent)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2* This,
        UINT16 scanCode,
        enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CRemoteKeyEventAttributes attributes);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_ReportPredictedKeyEvent(This, scanCode, attributes) \
    ((This)->lpVtbl->ReportPredictedKeyEvent(This, scanCode, attributes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Input_IRemoteTextConnectionFactory[] = L"Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory* This,
        GUID connectionId,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler* pduForwarder,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_CreateInstance(This, connectionId, pduForwarder, value) \
    ((This)->lpVtbl->CreateInstance(This, connectionId, pduForwarder, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Input_IRemoteTextConnectionFactory2[] = L"Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2* This,
        GUID connectionId,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionDataHandler* pduForwarder,
        enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CRemoteTextConnectionOptions options,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_CreateInstance(This, connectionId, pduForwarder, options, value) \
    ((This)->lpVtbl->CreateInstance(This, connectionId, pduForwarder, options, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CInput_CIRemoteTextConnectionFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.System.RemoteDesktop.Input.RemoteTextConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory2 interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Input.IRemoteTextConnection ** Default Interface **
 *    Windows.System.RemoteDesktop.Input.IRemoteTextConnection2
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Input_RemoteTextConnection_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Input_RemoteTextConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Input_RemoteTextConnection[] = L"Windows.System.RemoteDesktop.Input.RemoteTextConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Eremotedesktop2Einput_p_h__

#endif // __windows2Esystem2Eremotedesktop2Einput_h__
