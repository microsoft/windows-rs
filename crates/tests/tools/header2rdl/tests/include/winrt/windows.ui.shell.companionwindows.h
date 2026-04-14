
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
#ifndef __windows2Eui2Eshell2Ecompanionwindows_h__
#define __windows2Eui2Eshell2Ecompanionwindows_h__
#ifndef __windows2Eui2Eshell2Ecompanionwindows_p_h__
#define __windows2Eui2Eshell2Ecompanionwindows_p_h__


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

#if !defined(WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION)
#define WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.UI.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    interface ICompanionWindowCoordinator;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowCoordinator

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    interface ICompanionWindowCoordinatorStatics;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowCoordinatorStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    interface ICompanionWindowRequest;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowRequest

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    interface ICompanionWindowRequestResult;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowRequestResult

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    interface ICompanionWindowRequestStatics;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowRequestStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    class CompanionWindowRequestResult;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("71d91c43-b325-5c8a-815e-168c1119a2e6"))
IAsyncOperation<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowRequestResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowRequestResult*, ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowRequestResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowRequestResult*> __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_t;
#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_USE */

#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f9532e40-aa97-5a3d-9d0f-42906673448b"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowRequestResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowRequestResult*, ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowRequestResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowRequestResult*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_USE */

#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    class CompanionWindowCoordinator;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c7413260-4a0d-50d6-bbf8-5f0854315716"))
ITypedEventHandler<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowCoordinator*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowCoordinator*, ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowCoordinator*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowCoordinator*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_USE */

#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    typedef enum CompanionWindowRequestResultStatus : int CompanionWindowRequestResultStatus;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    class CompanionWindowRequest;
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResultStatus
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    enum CompanionWindowRequestResultStatus : int
                    {
                        CompanionWindowRequestResultStatus_Success = 0,
                        CompanionWindowRequestResultStatus_UnknownFailure = 1,
                        CompanionWindowRequestResultStatus_RegistrationNotFound = 2,
                        CompanionWindowRequestResultStatus_ActivationTimedOut = 3,
                        CompanionWindowRequestResultStatus_RejectedByCompanionApp = 4,
                    };
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinator
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowCoordinator[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinator";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    MIDL_INTERFACE("05620e87-b0f7-59ba-b3a5-d614bdc1ebe3")
                    ICompanionWindowCoordinator : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestWindowFromAppAsync(
                            HSTRING appId,
                            __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DetachCompanionWindow(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompanionWindowId(
                            ABI::Windows::UI::WindowId* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Changed(
                            __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICompanionWindowCoordinator = __uuidof(ICompanionWindowCoordinator);
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinatorStatics
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowCoordinatorStatics[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinatorStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    MIDL_INTERFACE("964022fa-380e-518c-bfc8-0f3b84fafea3")
                    ICompanionWindowCoordinatorStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForWindow(
                            ABI::Windows::UI::WindowId windowId,
                            ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowCoordinator** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICompanionWindowCoordinatorStatics = __uuidof(ICompanionWindowCoordinatorStatics);
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowRequest
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowRequest
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowRequest[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowRequest";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    MIDL_INTERFACE("d92c351a-2d66-59a8-b345-78489562c4d8")
                    ICompanionWindowRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Accept(
                            ABI::Windows::UI::WindowId windowId,
                            ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowCoordinator** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Reject(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RequestingWindowId(
                            ABI::Windows::UI::WindowId* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICompanionWindowRequest = __uuidof(ICompanionWindowRequest);
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestResult
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowRequestResult[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestResult";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    MIDL_INTERFACE("d728d2ef-e6d4-5cc0-9ff4-20c17a2ce72d")
                    ICompanionWindowRequestResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::UI::Shell::CompanionWindows::CompanionWindowRequestResultStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompanionWindowId(
                            ABI::Windows::UI::WindowId* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICompanionWindowRequestResult = __uuidof(ICompanionWindowRequestResult);
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestStatics
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowRequest
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowRequestStatics[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace CompanionWindows {
                    MIDL_INTERFACE("585e4544-d474-506a-96c2-3597a44882da")
                    ICompanionWindowRequestStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetFromLaunchUri(
                            ABI::Windows::Foundation::IUriRuntimeClass* launchUri,
                            ABI::Windows::UI::Shell::CompanionWindows::ICompanionWindowRequest** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICompanionWindowRequestStatics = __uuidof(ICompanionWindowRequestStatics);
                } /* CompanionWindows */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinatorStatics interface starting with version 1.0 of the Windows.UI.Shell.CompanionWindows.CompanionWindowsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowCoordinator_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowCoordinator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_CompanionWindows_CompanionWindowCoordinator[] = L"Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator";
#endif
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.CompanionWindows.CompanionWindowRequest
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestStatics interface starting with version 1.0 of the Windows.UI.Shell.CompanionWindows.CompanionWindowsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.CompanionWindows.ICompanionWindowRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_CompanionWindows_CompanionWindowRequest[] = L"Windows.UI.Shell.CompanionWindows.CompanionWindowRequest";
#endif
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowRequestResult_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowRequestResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_CompanionWindows_CompanionWindowRequestResult[] = L"Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult";
#endif
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator;

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest;

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult;

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult;

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult;

typedef struct __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResultVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* This,
        __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable* This,
        __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

typedef enum __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CCompanionWindowRequestResultStatus __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CCompanionWindowRequestResultStatus;

/*
 *
 * Struct Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResultStatus
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CCompanionWindowRequestResultStatus
{
    CompanionWindowRequestResultStatus_Success = 0,
    CompanionWindowRequestResultStatus_UnknownFailure = 1,
    CompanionWindowRequestResultStatus_RegistrationNotFound = 2,
    CompanionWindowRequestResultStatus_ActivationTimedOut = 3,
    CompanionWindowRequestResultStatus_RejectedByCompanionApp = 4,
};
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinator
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowCoordinator[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinator";
typedef struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestWindowFromAppAsync)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This,
        HSTRING appId,
        __FIAsyncOperation_1_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowRequestResult** operation);
    HRESULT (STDMETHODCALLTYPE* DetachCompanionWindow)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This);
    HRESULT (STDMETHODCALLTYPE* get_CompanionWindowId)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This,
        struct __x_ABI_CWindows_CUI_CWindowId* value);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This,
        __FITypedEventHandler_2_Windows__CUI__CShell__CCompanionWindows__CCompanionWindowCoordinator_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorVtbl;

interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_RequestWindowFromAppAsync(This, appId, operation) \
    ((This)->lpVtbl->RequestWindowFromAppAsync(This, appId, operation))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_DetachCompanionWindow(This) \
    ((This)->lpVtbl->DetachCompanionWindow(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_get_CompanionWindowId(This, value) \
    ((This)->lpVtbl->get_CompanionWindowId(This, value))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinatorStatics
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowCoordinatorStatics[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinatorStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForWindow)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics* This,
        struct __x_ABI_CWindows_CUI_CWindowId windowId,
        __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_GetForWindow(This, windowId, result) \
    ((This)->lpVtbl->GetForWindow(This, windowId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowRequest
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowRequest
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowRequest[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowRequest";
typedef struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Accept)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This,
        struct __x_ABI_CWindows_CUI_CWindowId windowId,
        __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowCoordinator** result);
    HRESULT (STDMETHODCALLTYPE* Reject)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);
    HRESULT (STDMETHODCALLTYPE* get_RequestingWindowId)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest* This,
        struct __x_ABI_CWindows_CUI_CWindowId* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestVtbl;

interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_Accept(This, windowId, result) \
    ((This)->lpVtbl->Accept(This, windowId, result))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_Reject(This) \
    ((This)->lpVtbl->Reject(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_get_RequestingWindowId(This, value) \
    ((This)->lpVtbl->get_RequestingWindowId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestResult
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowRequestResult[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestResult";
typedef struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This,
        enum __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CCompanionWindowRequestResultStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_CompanionWindowId)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult* This,
        struct __x_ABI_CWindows_CUI_CWindowId* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResultVtbl;

interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_get_CompanionWindowId(This, value) \
    ((This)->lpVtbl->get_CompanionWindowId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestStatics
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.CompanionWindows.CompanionWindowRequest
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_CompanionWindows_ICompanionWindowRequestStatics[] = L"Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFromLaunchUri)(__x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* launchUri,
        __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequest** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_GetFromLaunchUri(This, launchUri, result) \
    ((This)->lpVtbl->GetFromLaunchUri(This, launchUri, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CCompanionWindows_CICompanionWindowRequestStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinatorStatics interface starting with version 1.0 of the Windows.UI.Shell.CompanionWindows.CompanionWindowsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.CompanionWindows.ICompanionWindowCoordinator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowCoordinator_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowCoordinator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_CompanionWindows_CompanionWindowCoordinator[] = L"Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator";
#endif
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.CompanionWindows.CompanionWindowRequest
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestStatics interface starting with version 1.0 of the Windows.UI.Shell.CompanionWindows.CompanionWindowsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.CompanionWindows.ICompanionWindowRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_CompanionWindows_CompanionWindowRequest[] = L"Windows.UI.Shell.CompanionWindows.CompanionWindowRequest";
#endif
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult
 *
 * Introduced to Windows.UI.Shell.CompanionWindows.CompanionWindowsContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.CompanionWindows.ICompanionWindowRequestResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowRequestResult_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_CompanionWindows_CompanionWindowRequestResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_CompanionWindows_CompanionWindowRequestResult[] = L"Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult";
#endif
#endif // WINDOWS_UI_SHELL_COMPANIONWINDOWS_COMPANIONWINDOWSCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Eshell2Ecompanionwindows_p_h__

#endif // __windows2Eui2Eshell2Ecompanionwindows_h__
