
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
#ifndef __windows2Eperception2Eautomation2Ecore_h__
#define __windows2Eperception2Eautomation2Ecore_h__
#ifndef __windows2Eperception2Eautomation2Ecore_p_h__
#define __windows2Eperception2Eautomation2Ecore_p_h__


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

#if !defined(WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION)
#define WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Automation {
                namespace Core {
                    interface ICorePerceptionAutomationStatics;
                } /* Core */
            } /* Automation */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics ABI::Windows::Perception::Automation::Core::ICorePerceptionAutomationStatics

#endif // ____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IGetActivationFactory;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory ABI::Windows::Foundation::IGetActivationFactory

#endif // ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__

/*
 *
 * Interface Windows.Perception.Automation.Core.ICorePerceptionAutomationStatics
 *
 * Introduced to Windows.Perception.Automation.Core.PerceptionAutomationCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Automation.Core.CorePerceptionAutomation
 *
 */
#if WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Automation_Core_ICorePerceptionAutomationStatics[] = L"Windows.Perception.Automation.Core.ICorePerceptionAutomationStatics";
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Automation {
                namespace Core {
                    MIDL_INTERFACE("0bb04541-4ce2-4923-9a76-8187ecc59112")
                    ICorePerceptionAutomationStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetActivationFactoryProvider(
                            ABI::Windows::Foundation::IGetActivationFactory* provider
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICorePerceptionAutomationStatics = __uuidof(ICorePerceptionAutomationStatics);
                } /* Core */
            } /* Automation */
        } /* Perception */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Perception.Automation.Core.CorePerceptionAutomation
 *
 * Introduced to Windows.Perception.Automation.Core.PerceptionAutomationCoreContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Automation.Core.ICorePerceptionAutomationStatics interface starting with version 1.0 of the Windows.Perception.Automation.Core.PerceptionAutomationCoreContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Perception_Automation_Core_CorePerceptionAutomation_DEFINED
#define RUNTIMECLASS_Windows_Perception_Automation_Core_CorePerceptionAutomation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Automation_Core_CorePerceptionAutomation[] = L"Windows.Perception.Automation.Core.CorePerceptionAutomation";
#endif
#endif // WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics;

#endif // ____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIGetActivationFactory __x_ABI_CWindows_CFoundation_CIGetActivationFactory;

#endif // ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__

/*
 *
 * Interface Windows.Perception.Automation.Core.ICorePerceptionAutomationStatics
 *
 * Introduced to Windows.Perception.Automation.Core.PerceptionAutomationCoreContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Perception.Automation.Core.CorePerceptionAutomation
 *
 */
#if WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Perception_Automation_Core_ICorePerceptionAutomationStatics[] = L"Windows.Perception.Automation.Core.ICorePerceptionAutomationStatics";
typedef struct __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetActivationFactoryProvider)(__x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics* This,
        __x_ABI_CWindows_CFoundation_CIGetActivationFactory* provider);

    END_INTERFACE
} __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStaticsVtbl;

interface __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_SetActivationFactoryProvider(This, provider) \
    ((This)->lpVtbl->SetActivationFactoryProvider(This, provider))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics;
#endif /* !defined(____x_ABI_CWindows_CPerception_CAutomation_CCore_CICorePerceptionAutomationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Perception.Automation.Core.CorePerceptionAutomation
 *
 * Introduced to Windows.Perception.Automation.Core.PerceptionAutomationCoreContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Perception.Automation.Core.ICorePerceptionAutomationStatics interface starting with version 1.0 of the Windows.Perception.Automation.Core.PerceptionAutomationCoreContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Perception_Automation_Core_CorePerceptionAutomation_DEFINED
#define RUNTIMECLASS_Windows_Perception_Automation_Core_CorePerceptionAutomation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Perception_Automation_Core_CorePerceptionAutomation[] = L"Windows.Perception.Automation.Core.CorePerceptionAutomation";
#endif
#endif // WINDOWS_PERCEPTION_AUTOMATION_CORE_PERCEPTIONAUTOMATIONCORECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eperception2Eautomation2Ecore_p_h__

#endif // __windows2Eperception2Eautomation2Ecore_h__
