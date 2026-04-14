
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
#ifndef __windows2Eapplicationmodel2Euseractivities2Ecore_h__
#define __windows2Eapplicationmodel2Euseractivities2Ecore_h__
#ifndef __windows2Eapplicationmodel2Euseractivities2Ecore_p_h__
#define __windows2Eapplicationmodel2Euseractivities2Ecore_p_h__


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
#include "Windows.ApplicationModel.UserActivities.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserActivities {
                namespace Core {
                    interface ICoreUserActivityManagerStatics;
                } /* Core */
            } /* UserActivities */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics ABI::Windows::ApplicationModel::UserActivities::Core::ICoreUserActivityManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserActivities {
                class UserActivity;
            } /* UserActivities */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserActivities {
                interface IUserActivity;
            } /* UserActivities */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity ABI::Windows::ApplicationModel::UserActivities::IUserActivity

#endif // ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserActivities {
                class UserActivityChannel;
            } /* UserActivities */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserActivities {
                interface IUserActivityChannel;
            } /* UserActivities */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel ABI::Windows::ApplicationModel::UserActivities::IUserActivityChannel

#endif // ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserActivities {
                class UserActivitySession;
            } /* UserActivities */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserActivities {
                interface IUserActivitySession;
            } /* UserActivities */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession ABI::Windows::ApplicationModel::UserActivities::IUserActivitySession

#endif // ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncAction;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncAction ABI::Windows::Foundation::IAsyncAction

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.UserActivities.Core.ICoreUserActivityManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserActivities_Core_ICoreUserActivityManagerStatics[] = L"Windows.ApplicationModel.UserActivities.Core.ICoreUserActivityManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserActivities {
                namespace Core {
                    MIDL_INTERFACE("ca3adb02-a4be-4d4d-bfa8-6795f4264efb")
                    ICoreUserActivityManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateUserActivitySessionInBackground(
                            ABI::Windows::ApplicationModel::UserActivities::IUserActivity* activity,
                            ABI::Windows::ApplicationModel::UserActivities::IUserActivitySession** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DeleteUserActivitySessionsInTimeRangeAsync(
                            ABI::Windows::ApplicationModel::UserActivities::IUserActivityChannel* channel,
                            ABI::Windows::Foundation::DateTime startTime,
                            ABI::Windows::Foundation::DateTime endTime,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreUserActivityManagerStatics = __uuidof(ICoreUserActivityManagerStatics);
                } /* Core */
            } /* UserActivities */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.UserActivities.Core.ICoreUserActivityManagerStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserActivities_Core_CoreUserActivityManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserActivities_Core_CoreUserActivityManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserActivities_Core_CoreUserActivityManager[] = L"Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.UserActivities.Core.ICoreUserActivityManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserActivities_Core_ICoreUserActivityManagerStatics[] = L"Windows.ApplicationModel.UserActivities.Core.ICoreUserActivityManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateUserActivitySessionInBackground)(__x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivity* activity,
        __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivitySession** result);
    HRESULT (STDMETHODCALLTYPE* DeleteUserActivitySessionsInTimeRangeAsync)(__x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CUserActivities_CIUserActivityChannel* channel,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        struct __x_ABI_CWindows_CFoundation_CDateTime endTime,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_CreateUserActivitySessionInBackground(This, activity, result) \
    ((This)->lpVtbl->CreateUserActivitySessionInBackground(This, activity, result))

#define __x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_DeleteUserActivitySessionsInTimeRangeAsync(This, channel, startTime, endTime, operation) \
    ((This)->lpVtbl->DeleteUserActivitySessionsInTimeRangeAsync(This, channel, startTime, endTime, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserActivities_CCore_CICoreUserActivityManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.UserActivities.Core.ICoreUserActivityManagerStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserActivities_Core_CoreUserActivityManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserActivities_Core_CoreUserActivityManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserActivities_Core_CoreUserActivityManager[] = L"Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Euseractivities2Ecore_p_h__

#endif // __windows2Eapplicationmodel2Euseractivities2Ecore_h__
