
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
#ifndef __windows2Esystem2Epower2Ediagnostics_h__
#define __windows2Esystem2Epower2Ediagnostics_h__
#ifndef __windows2Esystem2Epower2Ediagnostics_p_h__
#define __windows2Esystem2Epower2Ediagnostics_p_h__


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
#ifndef ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Diagnostics {
                    interface IBackgroundEnergyDiagnosticsStatics;
                } /* Diagnostics */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics ABI::Windows::System::Power::Diagnostics::IBackgroundEnergyDiagnosticsStatics

#endif // ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Diagnostics {
                    interface IForegroundEnergyDiagnosticsStatics;
                } /* Diagnostics */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics ABI::Windows::System::Power::Diagnostics::IForegroundEnergyDiagnosticsStatics

#endif // ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
/*
 *
 * Interface Windows.System.Power.Diagnostics.IBackgroundEnergyDiagnosticsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Diagnostics_IBackgroundEnergyDiagnosticsStatics[] = L"Windows.System.Power.Diagnostics.IBackgroundEnergyDiagnosticsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Diagnostics {
                    MIDL_INTERFACE("d7663702-d3a6-46e0-8f9b-50b95bb4f9c5")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    IBackgroundEnergyDiagnosticsStatics : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceSpecificConversionFactor(
                            DOUBLE* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        virtual HRESULT STDMETHODCALLTYPE ComputeTotalEnergyUsage(
                            UINT64* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        virtual HRESULT STDMETHODCALLTYPE ResetTotalEnergyUsage(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBackgroundEnergyDiagnosticsStatics = __uuidof(IBackgroundEnergyDiagnosticsStatics);
                } /* Diagnostics */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Diagnostics.IForegroundEnergyDiagnosticsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Diagnostics_IForegroundEnergyDiagnosticsStatics[] = L"Windows.System.Power.Diagnostics.IForegroundEnergyDiagnosticsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Power {
                namespace Diagnostics {
                    MIDL_INTERFACE("23ca0917-cd07-4609-be15-8fe894c5e41e")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    IForegroundEnergyDiagnosticsStatics : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceSpecificConversionFactor(
                            DOUBLE* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        virtual HRESULT STDMETHODCALLTYPE ComputeTotalEnergyUsage(
                            UINT64* value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        virtual HRESULT STDMETHODCALLTYPE ResetTotalEnergyUsage(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IForegroundEnergyDiagnosticsStatics = __uuidof(IForegroundEnergyDiagnosticsStatics);
                } /* Diagnostics */
            } /* Power */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.Diagnostics.IBackgroundEnergyDiagnosticsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Diagnostics_BackgroundEnergyDiagnostics_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Diagnostics_BackgroundEnergyDiagnostics_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Diagnostics_BackgroundEnergyDiagnostics[] = L"Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.Diagnostics.IForegroundEnergyDiagnosticsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Diagnostics_ForegroundEnergyDiagnostics_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Diagnostics_ForegroundEnergyDiagnostics_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Foreground Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Diagnostics_ForegroundEnergyDiagnostics[] = L"Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics;

#endif // ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics;

#endif // ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

/*
 *
 * Interface Windows.System.Power.Diagnostics.IBackgroundEnergyDiagnosticsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Diagnostics_IBackgroundEnergyDiagnosticsStatics[] = L"Windows.System.Power.Diagnostics.IBackgroundEnergyDiagnosticsStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_DeviceSpecificConversionFactor)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This,
        DOUBLE* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* ComputeTotalEnergyUsage)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This,
        UINT64* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* ResetTotalEnergyUsage)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_get_DeviceSpecificConversionFactor(This, value) \
    ((This)->lpVtbl->get_DeviceSpecificConversionFactor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_ComputeTotalEnergyUsage(This, value) \
    ((This)->lpVtbl->ComputeTotalEnergyUsage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_ResetTotalEnergyUsage(This) \
    ((This)->lpVtbl->ResetTotalEnergyUsage(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIBackgroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Power.Diagnostics.IForegroundEnergyDiagnosticsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Power_Diagnostics_IForegroundEnergyDiagnosticsStatics[] = L"Windows.System.Power.Diagnostics.IForegroundEnergyDiagnosticsStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* get_DeviceSpecificConversionFactor)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This,
        DOUBLE* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* ComputeTotalEnergyUsage)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This,
        UINT64* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    HRESULT (STDMETHODCALLTYPE* ResetTotalEnergyUsage)(__x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_get_DeviceSpecificConversionFactor(This, value) \
    ((This)->lpVtbl->get_DeviceSpecificConversionFactor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_ComputeTotalEnergyUsage(This, value) \
    ((This)->lpVtbl->ComputeTotalEnergyUsage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#define __x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_ResetTotalEnergyUsage(This) \
    ((This)->lpVtbl->ResetTotalEnergyUsage(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPower_CDiagnostics_CIForegroundEnergyDiagnosticsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.Diagnostics.IBackgroundEnergyDiagnosticsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Diagnostics_BackgroundEnergyDiagnostics_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Diagnostics_BackgroundEnergyDiagnostics_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Background Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Diagnostics_BackgroundEnergyDiagnostics[] = L"Windows.System.Power.Diagnostics.BackgroundEnergyDiagnostics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Power.Diagnostics.IForegroundEnergyDiagnosticsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Power_Diagnostics_ForegroundEnergyDiagnostics_DEFINED
#define RUNTIMECLASS_Windows_System_Power_Diagnostics_ForegroundEnergyDiagnostics_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
DEPRECATED("Foreground Energy Diagnostics has been deprecated. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Power_Diagnostics_ForegroundEnergyDiagnostics[] = L"Windows.System.Power.Diagnostics.ForegroundEnergyDiagnostics";
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
#endif // __windows2Esystem2Epower2Ediagnostics_p_h__

#endif // __windows2Esystem2Epower2Ediagnostics_h__
