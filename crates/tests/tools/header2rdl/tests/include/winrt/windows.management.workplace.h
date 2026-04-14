
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
#ifndef __windows2Emanagement2Eworkplace_h__
#define __windows2Emanagement2Eworkplace_h__
#ifndef __windows2Emanagement2Eworkplace_p_h__
#define __windows2Emanagement2Eworkplace_p_h__


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

#if !defined(WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION)
#define WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Workplace {
                interface IMdmAllowPolicyStatics;
            } /* Workplace */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics ABI::Windows::Management::Workplace::IMdmAllowPolicyStatics

#endif // ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Workplace {
                interface IMdmPolicyStatics2;
            } /* Workplace */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2 ABI::Windows::Management::Workplace::IMdmPolicyStatics2

#endif // ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Workplace {
                interface IWorkplaceSettingsStatics;
            } /* Workplace */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics ABI::Windows::Management::Workplace::IWorkplaceSettingsStatics

#endif // ____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Workplace {
                typedef enum MessagingSyncPolicy : int MessagingSyncPolicy;
            } /* Workplace */
        } /* Management */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Management.Workplace.MessagingSyncPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Workplace {
                enum MessagingSyncPolicy : int
                {
                    MessagingSyncPolicy_Disallowed = 0,
                    MessagingSyncPolicy_Allowed = 1,
                    MessagingSyncPolicy_Required = 2,
                };
            } /* Workplace */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Management.Workplace.IMdmAllowPolicyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Workplace.MdmPolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Workplace_IMdmAllowPolicyStatics[] = L"Windows.Management.Workplace.IMdmAllowPolicyStatics";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Workplace {
                MIDL_INTERFACE("c39709e7-741c-41f2-a4b6-314c31502586")
                IMdmAllowPolicyStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsBrowserAllowed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsCameraAllowed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsMicrosoftAccountAllowed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsStoreAllowed(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMdmAllowPolicyStatics = __uuidof(IMdmAllowPolicyStatics);
            } /* Workplace */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Workplace.IMdmPolicyStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Management.Workplace.MdmPolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Workplace_IMdmPolicyStatics2[] = L"Windows.Management.Workplace.IMdmPolicyStatics2";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Workplace {
                MIDL_INTERFACE("c99c7526-03d4-49f9-a993-43efccd265c4")
                IMdmPolicyStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetMessagingSyncPolicy(
                        ABI::Windows::Management::Workplace::MessagingSyncPolicy* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMdmPolicyStatics2 = __uuidof(IMdmPolicyStatics2);
            } /* Workplace */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2;
#endif /* !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Management.Workplace.IWorkplaceSettingsStatics
 *
 * Introduced to Windows.Management.Workplace.WorkplaceSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Workplace.WorkplaceSettings
 *
 */
#if WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Workplace_IWorkplaceSettingsStatics[] = L"Windows.Management.Workplace.IWorkplaceSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Workplace {
                MIDL_INTERFACE("e4676ffd-2d92-4c08-bad4-f6590b54a6d3")
                IWorkplaceSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsMicrosoftAccountOptional(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWorkplaceSettingsStatics = __uuidof(IWorkplaceSettingsStatics);
            } /* Workplace */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Workplace.MdmPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Workplace.IMdmAllowPolicyStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Management.Workplace.IMdmPolicyStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Workplace_MdmPolicy_DEFINED
#define RUNTIMECLASS_Windows_Management_Workplace_MdmPolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Workplace_MdmPolicy[] = L"Windows.Management.Workplace.MdmPolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Workplace.WorkplaceSettings
 *
 * Introduced to Windows.Management.Workplace.WorkplaceSettingsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Workplace.IWorkplaceSettingsStatics interface starting with version 1.0 of the Windows.Management.Workplace.WorkplaceSettingsContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Workplace_WorkplaceSettings_DEFINED
#define RUNTIMECLASS_Windows_Management_Workplace_WorkplaceSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Workplace_WorkplaceSettings[] = L"Windows.Management.Workplace.WorkplaceSettings";
#endif
#endif // WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics;

#endif // ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2 __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2;

#endif // ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics;

#endif // ____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CManagement_CWorkplace_CMessagingSyncPolicy __x_ABI_CWindows_CManagement_CWorkplace_CMessagingSyncPolicy;

/*
 *
 * Struct Windows.Management.Workplace.MessagingSyncPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CManagement_CWorkplace_CMessagingSyncPolicy
{
    MessagingSyncPolicy_Disallowed = 0,
    MessagingSyncPolicy_Allowed = 1,
    MessagingSyncPolicy_Required = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Management.Workplace.IMdmAllowPolicyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Workplace.MdmPolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Workplace_IMdmAllowPolicyStatics[] = L"Windows.Management.Workplace.IMdmAllowPolicyStatics";
typedef struct __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsBrowserAllowed)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsCameraAllowed)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsMicrosoftAccountAllowed)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsStoreAllowed)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStaticsVtbl;

interface __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_IsBrowserAllowed(This, value) \
    ((This)->lpVtbl->IsBrowserAllowed(This, value))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_IsCameraAllowed(This, value) \
    ((This)->lpVtbl->IsCameraAllowed(This, value))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_IsMicrosoftAccountAllowed(This, value) \
    ((This)->lpVtbl->IsMicrosoftAccountAllowed(This, value))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_IsStoreAllowed(This, value) \
    ((This)->lpVtbl->IsStoreAllowed(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIMdmAllowPolicyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Workplace.IMdmPolicyStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Management.Workplace.MdmPolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Workplace_IMdmPolicyStatics2[] = L"Windows.Management.Workplace.IMdmPolicyStatics2";
typedef struct __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetMessagingSyncPolicy)(__x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2* This,
        enum __x_ABI_CWindows_CManagement_CWorkplace_CMessagingSyncPolicy* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2Vtbl;

interface __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_GetMessagingSyncPolicy(This, value) \
    ((This)->lpVtbl->GetMessagingSyncPolicy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2;
#endif /* !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIMdmPolicyStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Management.Workplace.IWorkplaceSettingsStatics
 *
 * Introduced to Windows.Management.Workplace.WorkplaceSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Workplace.WorkplaceSettings
 *
 */
#if WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Workplace_IWorkplaceSettingsStatics[] = L"Windows.Management.Workplace.IWorkplaceSettingsStatics";
typedef struct __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsMicrosoftAccountOptional)(__x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStaticsVtbl;

interface __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_get_IsMicrosoftAccountOptional(This, value) \
    ((This)->lpVtbl->get_IsMicrosoftAccountOptional(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CWorkplace_CIWorkplaceSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Workplace.MdmPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Workplace.IMdmAllowPolicyStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Management.Workplace.IMdmPolicyStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Workplace_MdmPolicy_DEFINED
#define RUNTIMECLASS_Windows_Management_Workplace_MdmPolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Workplace_MdmPolicy[] = L"Windows.Management.Workplace.MdmPolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Workplace.WorkplaceSettings
 *
 * Introduced to Windows.Management.Workplace.WorkplaceSettingsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Workplace.IWorkplaceSettingsStatics interface starting with version 1.0 of the Windows.Management.Workplace.WorkplaceSettingsContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Workplace_WorkplaceSettings_DEFINED
#define RUNTIMECLASS_Windows_Management_Workplace_WorkplaceSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Workplace_WorkplaceSettings[] = L"Windows.Management.Workplace.WorkplaceSettings";
#endif
#endif // WINDOWS_MANAGEMENT_WORKPLACE_WORKPLACESETTINGSCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emanagement2Eworkplace_p_h__

#endif // __windows2Emanagement2Eworkplace_h__
