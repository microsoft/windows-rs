
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
#ifndef __windows2Esystem2Ediagnostics2Etelemetry_h__
#define __windows2Esystem2Ediagnostics2Etelemetry_h__
#ifndef __windows2Esystem2Ediagnostics2Etelemetry_p_h__
#define __windows2Esystem2Ediagnostics2Etelemetry_p_h__


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
#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    interface IPlatformTelemetryClientStatics;
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics ABI::Windows::System::Diagnostics::Telemetry::IPlatformTelemetryClientStatics

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    interface IPlatformTelemetryRegistrationResult;
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult ABI::Windows::System::Diagnostics::Telemetry::IPlatformTelemetryRegistrationResult

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    interface IPlatformTelemetryRegistrationSettings;
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings ABI::Windows::System::Diagnostics::Telemetry::IPlatformTelemetryRegistrationSettings

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    typedef enum PlatformTelemetryRegistrationStatus : int PlatformTelemetryRegistrationStatus;
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    class PlatformTelemetryRegistrationResult;
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    class PlatformTelemetryRegistrationSettings;
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    enum PlatformTelemetryRegistrationStatus : int
                    {
                        PlatformTelemetryRegistrationStatus_Success = 0,
                        PlatformTelemetryRegistrationStatus_SettingsOutOfRange = 1,
                        PlatformTelemetryRegistrationStatus_UnknownFailure = 2,
                    };
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.Telemetry.IPlatformTelemetryClientStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_Telemetry_IPlatformTelemetryClientStatics[] = L"Windows.System.Diagnostics.Telemetry.IPlatformTelemetryClientStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    MIDL_INTERFACE("9bf3f25d-d5c3-4eea-8dbe-9c8dbb0d9d8f")
                    IPlatformTelemetryClientStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Register(
                            HSTRING id,
                            ABI::Windows::System::Diagnostics::Telemetry::IPlatformTelemetryRegistrationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RegisterWithSettings(
                            HSTRING id,
                            ABI::Windows::System::Diagnostics::Telemetry::IPlatformTelemetryRegistrationSettings* settings,
                            ABI::Windows::System::Diagnostics::Telemetry::IPlatformTelemetryRegistrationResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlatformTelemetryClientStatics = __uuidof(IPlatformTelemetryClientStatics);
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_Telemetry_IPlatformTelemetryRegistrationResult[] = L"Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationResult";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    MIDL_INTERFACE("4d8518ab-2292-49bd-a15a-3d71d2145112")
                    IPlatformTelemetryRegistrationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::System::Diagnostics::Telemetry::PlatformTelemetryRegistrationStatus* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlatformTelemetryRegistrationResult = __uuidof(IPlatformTelemetryRegistrationResult);
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_Telemetry_IPlatformTelemetryRegistrationSettings[] = L"Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationSettings";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Diagnostics {
                namespace Telemetry {
                    MIDL_INTERFACE("819a8582-ca19-415e-bb79-9c224bfa3a73")
                    IPlatformTelemetryRegistrationSettings : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_StorageSize(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StorageSize(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UploadQuotaSize(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UploadQuotaSize(
                            UINT32 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlatformTelemetryRegistrationSettings = __uuidof(IPlatformTelemetryRegistrationSettings);
                } /* Telemetry */
            } /* Diagnostics */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.Telemetry.IPlatformTelemetryClientStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryClient_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryClient_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_Telemetry_PlatformTelemetryClient[] = L"Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationResult_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationResult[] = L"Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationSettings_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationSettings[] = L"Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings;

#endif // ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CPlatformTelemetryRegistrationStatus __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CPlatformTelemetryRegistrationStatus;

/*
 *
 * Struct Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CPlatformTelemetryRegistrationStatus
{
    PlatformTelemetryRegistrationStatus_Success = 0,
    PlatformTelemetryRegistrationStatus_SettingsOutOfRange = 1,
    PlatformTelemetryRegistrationStatus_UnknownFailure = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.Telemetry.IPlatformTelemetryClientStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_Telemetry_IPlatformTelemetryClientStatics[] = L"Windows.System.Diagnostics.Telemetry.IPlatformTelemetryClientStatics";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Register)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics* This,
        HSTRING id,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult** result);
    HRESULT (STDMETHODCALLTYPE* RegisterWithSettings)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics* This,
        HSTRING id,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* settings,
        __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_Register(This, id, result) \
    ((This)->lpVtbl->Register(This, id, result))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_RegisterWithSettings(This, id, settings, result) \
    ((This)->lpVtbl->RegisterWithSettings(This, id, settings, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryClientStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_Telemetry_IPlatformTelemetryRegistrationResult[] = L"Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationResult";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult* This,
        enum __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CPlatformTelemetryRegistrationStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResultVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Diagnostics_Telemetry_IPlatformTelemetryRegistrationSettings[] = L"Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationSettings";
typedef struct __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StorageSize)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_StorageSize)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_UploadQuotaSize)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_UploadQuotaSize)(__x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettingsVtbl;

interface __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_get_StorageSize(This, value) \
    ((This)->lpVtbl->get_StorageSize(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_put_StorageSize(This, value) \
    ((This)->lpVtbl->put_StorageSize(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_get_UploadQuotaSize(This, value) \
    ((This)->lpVtbl->get_UploadQuotaSize(This, value))

#define __x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_put_UploadQuotaSize(This, value) \
    ((This)->lpVtbl->put_UploadQuotaSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CDiagnostics_CTelemetry_CIPlatformTelemetryRegistrationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Diagnostics.Telemetry.IPlatformTelemetryClientStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryClient_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryClient_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_Telemetry_PlatformTelemetryClient[] = L"Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationResult_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationResult[] = L"Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Diagnostics.Telemetry.IPlatformTelemetryRegistrationSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationSettings_DEFINED
#define RUNTIMECLASS_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Diagnostics_Telemetry_PlatformTelemetryRegistrationSettings[] = L"Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Ediagnostics2Etelemetry_p_h__

#endif // __windows2Esystem2Ediagnostics2Etelemetry_h__
