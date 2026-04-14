
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
#ifndef __windows2Emanagement2Edeployment2Epreview_h__
#define __windows2Emanagement2Edeployment2Epreview_h__
#ifndef __windows2Emanagement2Edeployment2Epreview_p_h__
#define __windows2Emanagement2Edeployment2Epreview_p_h__


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
#if !defined(WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION)
#define WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Deployment {
                namespace Preview {
                    interface IClassicAppManagerStatics;
                } /* Preview */
            } /* Deployment */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics ABI::Windows::Management::Deployment::Preview::IClassicAppManagerStatics

#endif // ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Deployment {
                namespace Preview {
                    interface IInstalledClassicAppInfo;
                } /* Preview */
            } /* Deployment */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo ABI::Windows::Management::Deployment::Preview::IInstalledClassicAppInfo

#endif // ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Deployment {
                namespace Preview {
                    class InstalledClassicAppInfo;
                } /* Preview */
            } /* Deployment */
        } /* Management */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Management.Deployment.Preview.IClassicAppManagerStatics
 *
 * Introduced to Windows.Management.Deployment.Preview.DeploymentPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Deployment.Preview.ClassicAppManager
 *
 */
#if WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Deployment_Preview_IClassicAppManagerStatics[] = L"Windows.Management.Deployment.Preview.IClassicAppManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Deployment {
                namespace Preview {
                    MIDL_INTERFACE("e2fad668-882c-4f33-b035-0df7b90d67e6")
                    IClassicAppManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FindInstalledApp(
                            HSTRING appUninstallKey,
                            ABI::Windows::Management::Deployment::Preview::IInstalledClassicAppInfo** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IClassicAppManagerStatics = __uuidof(IClassicAppManagerStatics);
                } /* Preview */
            } /* Deployment */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Deployment.Preview.IInstalledClassicAppInfo
 *
 * Introduced to Windows.Management.Deployment.Preview.DeploymentPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Deployment.Preview.InstalledClassicAppInfo
 *
 */
#if WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Deployment_Preview_IInstalledClassicAppInfo[] = L"Windows.Management.Deployment.Preview.IInstalledClassicAppInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Deployment {
                namespace Preview {
                    MIDL_INTERFACE("0a7d3da3-65d0-4086-80d6-0610d760207d")
                    IInstalledClassicAppInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayVersion(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInstalledClassicAppInfo = __uuidof(IInstalledClassicAppInfo);
                } /* Preview */
            } /* Deployment */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Deployment.Preview.ClassicAppManager
 *
 * Introduced to Windows.Management.Deployment.Preview.DeploymentPreviewContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Deployment.Preview.IClassicAppManagerStatics interface starting with version 1.0 of the Windows.Management.Deployment.Preview.DeploymentPreviewContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Deployment_Preview_ClassicAppManager_DEFINED
#define RUNTIMECLASS_Windows_Management_Deployment_Preview_ClassicAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Deployment_Preview_ClassicAppManager[] = L"Windows.Management.Deployment.Preview.ClassicAppManager";
#endif
#endif // WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Deployment.Preview.InstalledClassicAppInfo
 *
 * Introduced to Windows.Management.Deployment.Preview.DeploymentPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Deployment.Preview.IInstalledClassicAppInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Deployment_Preview_InstalledClassicAppInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Deployment_Preview_InstalledClassicAppInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Deployment_Preview_InstalledClassicAppInfo[] = L"Windows.Management.Deployment.Preview.InstalledClassicAppInfo";
#endif
#endif // WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics;

#endif // ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo;

#endif // ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

/*
 *
 * Interface Windows.Management.Deployment.Preview.IClassicAppManagerStatics
 *
 * Introduced to Windows.Management.Deployment.Preview.DeploymentPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Deployment.Preview.ClassicAppManager
 *
 */
#if WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Deployment_Preview_IClassicAppManagerStatics[] = L"Windows.Management.Deployment.Preview.IClassicAppManagerStatics";
typedef struct __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindInstalledApp)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics* This,
        HSTRING appUninstallKey,
        __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStaticsVtbl;

interface __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_FindInstalledApp(This, appUninstallKey, result) \
    ((This)->lpVtbl->FindInstalledApp(This, appUninstallKey, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIClassicAppManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Deployment.Preview.IInstalledClassicAppInfo
 *
 * Introduced to Windows.Management.Deployment.Preview.DeploymentPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Deployment.Preview.InstalledClassicAppInfo
 *
 */
#if WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Deployment_Preview_IInstalledClassicAppInfo[] = L"Windows.Management.Deployment.Preview.IInstalledClassicAppInfo";
typedef struct __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayVersion)(__x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfoVtbl;

interface __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_get_DisplayVersion(This, value) \
    ((This)->lpVtbl->get_DisplayVersion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CDeployment_CPreview_CIInstalledClassicAppInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Deployment.Preview.ClassicAppManager
 *
 * Introduced to Windows.Management.Deployment.Preview.DeploymentPreviewContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Deployment.Preview.IClassicAppManagerStatics interface starting with version 1.0 of the Windows.Management.Deployment.Preview.DeploymentPreviewContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Deployment_Preview_ClassicAppManager_DEFINED
#define RUNTIMECLASS_Windows_Management_Deployment_Preview_ClassicAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Deployment_Preview_ClassicAppManager[] = L"Windows.Management.Deployment.Preview.ClassicAppManager";
#endif
#endif // WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Deployment.Preview.InstalledClassicAppInfo
 *
 * Introduced to Windows.Management.Deployment.Preview.DeploymentPreviewContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Deployment.Preview.IInstalledClassicAppInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Deployment_Preview_InstalledClassicAppInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Deployment_Preview_InstalledClassicAppInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Deployment_Preview_InstalledClassicAppInfo[] = L"Windows.Management.Deployment.Preview.InstalledClassicAppInfo";
#endif
#endif // WINDOWS_MANAGEMENT_DEPLOYMENT_PREVIEW_DEPLOYMENTPREVIEWCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emanagement2Edeployment2Epreview_p_h__

#endif // __windows2Emanagement2Edeployment2Epreview_h__
