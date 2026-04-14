
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
#ifndef __windows2Eai2Eactions2Eprovider_h__
#define __windows2Eai2Eactions2Eprovider_h__
#ifndef __windows2Eai2Eactions2Eprovider_p_h__
#define __windows2Eai2Eactions2Eprovider_p_h__


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
#if !defined(WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION)
#define WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION 0x80000
#endif // defined(WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION)

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
#include "Windows.AI.Actions.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                namespace Provider {
                    interface IActionFeedbackHandler;
                } /* Provider */
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler ABI::Windows::AI::Actions::Provider::IActionFeedbackHandler

#endif // ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                namespace Provider {
                    interface IActionProvider;
                } /* Provider */
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider ABI::Windows::AI::Actions::Provider::IActionProvider

#endif // ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ActionFeedback;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionFeedback;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback ABI::Windows::AI::Actions::IActionFeedback

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ActionInvocationContext;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionInvocationContext;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext ABI::Windows::AI::Actions::IActionInvocationContext

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__

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
 * Interface Windows.AI.Actions.Provider.IActionFeedbackHandler
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_Provider_IActionFeedbackHandler[] = L"Windows.AI.Actions.Provider.IActionFeedbackHandler";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                namespace Provider {
                    MIDL_INTERFACE("a3fc3c51-a8c6-52c8-ad77-37bf3e2b565c")
                    IActionFeedbackHandler : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ProcessFeedbackAsync(
                            ABI::Windows::AI::Actions::IActionInvocationContext* context,
                            ABI::Windows::AI::Actions::IActionFeedback* feedback,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IActionFeedbackHandler = __uuidof(IActionFeedbackHandler);
                } /* Provider */
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.Provider.IActionProvider
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_Provider_IActionProvider[] = L"Windows.AI.Actions.Provider.IActionProvider";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                namespace Provider {
                    MIDL_INTERFACE("62906c47-3d07-55f1-aefa-1522505afbbe")
                    IActionProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE InvokeAsync(
                            ABI::Windows::AI::Actions::IActionInvocationContext* context,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IActionProvider = __uuidof(IActionProvider);
                } /* Provider */
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler;

#endif // ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider;

#endif // ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionFeedback __x_ABI_CWindows_CAI_CActions_CIActionFeedback;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

/*
 *
 * Interface Windows.AI.Actions.Provider.IActionFeedbackHandler
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_Provider_IActionFeedbackHandler[] = L"Windows.AI.Actions.Provider.IActionFeedbackHandler";
typedef struct __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ProcessFeedbackAsync)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler* This,
        __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* context,
        __x_ABI_CWindows_CAI_CActions_CIActionFeedback* feedback,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandlerVtbl;

interface __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_ProcessFeedbackAsync(This, context, feedback, operation) \
    ((This)->lpVtbl->ProcessFeedbackAsync(This, context, feedback, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CProvider_CIActionFeedbackHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.Provider.IActionProvider
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_Provider_IActionProvider[] = L"Windows.AI.Actions.Provider.IActionProvider";
typedef struct __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* InvokeAsync)(__x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider* This,
        __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* context,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProviderVtbl;

interface __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_InvokeAsync(This, context, operation) \
    ((This)->lpVtbl->InvokeAsync(This, context, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CProvider_CIActionProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eai2Eactions2Eprovider_p_h__

#endif // __windows2Eai2Eactions2Eprovider_h__
