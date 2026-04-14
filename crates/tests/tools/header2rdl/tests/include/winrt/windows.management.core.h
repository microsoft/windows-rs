
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
#ifndef __windows2Emanagement2Ecore_h__
#define __windows2Emanagement2Ecore_h__
#ifndef __windows2Emanagement2Ecore_p_h__
#define __windows2Emanagement2Ecore_p_h__


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
#include "Windows.Storage.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Core {
                interface IApplicationDataManager;
            } /* Core */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager ABI::Windows::Management::Core::IApplicationDataManager

#endif // ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Core {
                interface IApplicationDataManagerStatics;
            } /* Core */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics ABI::Windows::Management::Core::IApplicationDataManagerStatics

#endif // ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Storage {
            class ApplicationData;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IApplicationData;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIApplicationData ABI::Windows::Storage::IApplicationData

#endif // ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__

/*
 *
 * Interface Windows.Management.Core.IApplicationDataManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Core.ApplicationDataManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Core_IApplicationDataManager[] = L"Windows.Management.Core.IApplicationDataManager";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Core {
                MIDL_INTERFACE("74d10432-2e99-4000-9a3a-64307e858129")
                IApplicationDataManager : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IApplicationDataManager = __uuidof(IApplicationDataManager);
            } /* Core */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager;
#endif /* !defined(____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Core.IApplicationDataManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Core.ApplicationDataManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Core_IApplicationDataManagerStatics[] = L"Windows.Management.Core.IApplicationDataManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Core {
                MIDL_INTERFACE("1e1862e3-698e-49a1-9752-dee94925b9b3")
                IApplicationDataManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForPackageFamily(
                        HSTRING packageFamilyName,
                        ABI::Windows::Storage::IApplicationData** applicationData
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationDataManagerStatics = __uuidof(IApplicationDataManagerStatics);
            } /* Core */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Core.ApplicationDataManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Core.IApplicationDataManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Core.IApplicationDataManager ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Core_ApplicationDataManager_DEFINED
#define RUNTIMECLASS_Windows_Management_Core_ApplicationDataManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Core_ApplicationDataManager[] = L"Windows.Management.Core.ApplicationDataManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager;

#endif // ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics;

#endif // ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIApplicationData __x_ABI_CWindows_CStorage_CIApplicationData;

#endif // ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__

/*
 *
 * Interface Windows.Management.Core.IApplicationDataManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Core.ApplicationDataManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Core_IApplicationDataManager[] = L"Windows.Management.Core.IApplicationDataManager";
typedef struct __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerVtbl;

interface __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager;
#endif /* !defined(____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Core.IApplicationDataManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Core.ApplicationDataManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Core_IApplicationDataManagerStatics[] = L"Windows.Management.Core.IApplicationDataManagerStatics";
typedef struct __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForPackageFamily)(__x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics* This,
        HSTRING packageFamilyName,
        __x_ABI_CWindows_CStorage_CIApplicationData** applicationData);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStaticsVtbl;

interface __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_CreateForPackageFamily(This, packageFamilyName, applicationData) \
    ((This)->lpVtbl->CreateForPackageFamily(This, packageFamilyName, applicationData))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CCore_CIApplicationDataManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Core.ApplicationDataManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Core.IApplicationDataManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Core.IApplicationDataManager ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Core_ApplicationDataManager_DEFINED
#define RUNTIMECLASS_Windows_Management_Core_ApplicationDataManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Core_ApplicationDataManager[] = L"Windows.Management.Core.ApplicationDataManager";
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
#endif // __windows2Emanagement2Ecore_p_h__

#endif // __windows2Emanagement2Ecore_h__
