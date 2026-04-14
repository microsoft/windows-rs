
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
#ifndef __windows2Eui2Euiautomation2Ecore_h__
#define __windows2Eui2Euiautomation2Ecore_h__
#ifndef __windows2Eui2Euiautomation2Ecore_p_h__
#define __windows2Eui2Euiautomation2Ecore_p_h__


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

#if !defined(WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION)
#define WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.UI.UIAutomation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface IAutomationRemoteOperationResult;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult ABI::Windows::UI::UIAutomation::Core::IAutomationRemoteOperationResult

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface ICoreAutomationConnectionBoundObjectProvider;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider ABI::Windows::UI::UIAutomation::Core::ICoreAutomationConnectionBoundObjectProvider

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface ICoreAutomationRegistrarStatics;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics ABI::Windows::UI::UIAutomation::Core::ICoreAutomationRegistrarStatics

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface ICoreAutomationRemoteOperation;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation ABI::Windows::UI::UIAutomation::Core::ICoreAutomationRemoteOperation

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface ICoreAutomationRemoteOperation2;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2 ABI::Windows::UI::UIAutomation::Core::ICoreAutomationRemoteOperation2

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface ICoreAutomationRemoteOperationContext;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext ABI::Windows::UI::UIAutomation::Core::ICoreAutomationRemoteOperationContext

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface ICoreAutomationRemoteOperationExtensionProvider;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider ABI::Windows::UI::UIAutomation::Core::ICoreAutomationRemoteOperationExtensionProvider

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface IRemoteAutomationClientSession;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationClientSession

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface IRemoteAutomationClientSessionFactory;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationClientSessionFactory

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface IRemoteAutomationConnectionRequestedEventArgs;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationConnectionRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface IRemoteAutomationDisconnectedEventArgs;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationDisconnectedEventArgs

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface IRemoteAutomationServerStatics;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationServerStatics

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    interface IRemoteAutomationWindow;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationWindow

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    class RemoteAutomationWindow;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3dc07f97-215c-5ad9-8f22-5c522d0f09ec"))
IAsyncOperation<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationWindow*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationWindow*, ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationWindow*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.UIAutomation.Core.RemoteAutomationWindow>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationWindow*> __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_t;
#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_USE */

#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9d069343-0252-551a-9abc-e234c736501f"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationWindow*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationWindow*, ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationWindow*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.UIAutomation.Core.RemoteAutomationWindow>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationWindow*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_USE */

#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    class RemoteAutomationClientSession;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    class RemoteAutomationConnectionRequestedEventArgs;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1b0eb0d9-b29b-52f6-9d79-fffe5a280299"))
ITypedEventHandler<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationClientSession*, ABI::Windows::UI::UIAutomation::Core::RemoteAutomationConnectionRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationClientSession*, ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationClientSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationConnectionRequestedEventArgs*, ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationConnectionRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.UIAutomation.Core.RemoteAutomationClientSession, Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationClientSession*, ABI::Windows::UI::UIAutomation::Core::RemoteAutomationConnectionRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_USE */

#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    class RemoteAutomationDisconnectedEventArgs;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7007e98a-48ed-52c8-a5b9-e1918b3f2709"))
ITypedEventHandler<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationClientSession*, ABI::Windows::UI::UIAutomation::Core::RemoteAutomationDisconnectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationClientSession*, ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationClientSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationDisconnectedEventArgs*, ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationDisconnectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.UIAutomation.Core.RemoteAutomationClientSession, Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::UIAutomation::Core::RemoteAutomationClientSession*, ABI::Windows::UI::UIAutomation::Core::RemoteAutomationDisconnectedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_USE */

#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

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

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                class AutomationConnectionBoundObject;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                interface IAutomationConnectionBoundObject;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject ABI::Windows::UI::UIAutomation::IAutomationConnectionBoundObject

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                class AutomationElement;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                interface IAutomationElement;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement ABI::Windows::UI::UIAutomation::IAutomationElement

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                class AutomationTextRange;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                interface IAutomationTextRange;
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange ABI::Windows::UI::UIAutomation::IAutomationTextRange

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    typedef enum AutomationRemoteOperationStatus : int AutomationRemoteOperationStatus;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    typedef struct AutomationAnnotationTypeRegistration AutomationAnnotationTypeRegistration;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    typedef struct AutomationRemoteOperationOperandId AutomationRemoteOperationOperandId;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    class AutomationRemoteOperationResult;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    class CoreAutomationRemoteOperationContext;
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.UIAutomation.Core.AutomationRemoteOperationStatus
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    enum AutomationRemoteOperationStatus : int
                    {
                        AutomationRemoteOperationStatus_Success = 0,
                        AutomationRemoteOperationStatus_MalformedBytecode = 1,
                        AutomationRemoteOperationStatus_InstructionLimitExceeded = 2,
                        AutomationRemoteOperationStatus_UnhandledException = 3,
                        AutomationRemoteOperationStatus_ExecutionFailure = 4,
                    };
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.UIAutomation.Core.AutomationAnnotationTypeRegistration
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    struct AutomationAnnotationTypeRegistration
                    {
                        INT32 LocalId;
                    };
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.UIAutomation.Core.AutomationRemoteOperationOperandId
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    struct AutomationRemoteOperationOperandId
                    {
                        INT32 Value;
                    };
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IAutomationRemoteOperationResult
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IAutomationRemoteOperationResult[] = L"Windows.UI.UIAutomation.Core.IAutomationRemoteOperationResult";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("e0f80c42-4a67-5534-bf5a-09e8a99b36b1")
                    IAutomationRemoteOperationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ErrorLocation(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE HasOperand(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId operandId,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetOperand(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId operandId,
                            IInspectable** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationRemoteOperationResult = __uuidof(IAutomationRemoteOperationResult);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationConnectionBoundObjectProvider[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("0620bb64-9616-5593-be3a-eb8e6daeb3fa")
                    ICoreAutomationConnectionBoundObjectProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsComThreadingRequired(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreAutomationConnectionBoundObjectProvider = __uuidof(ICoreAutomationConnectionBoundObjectProvider);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRegistrarStatics
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.CoreAutomationRegistrar
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRegistrarStatics[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRegistrarStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("3e50129b-d6dc-5680-b580-ffff78300304")
                    ICoreAutomationRegistrarStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RegisterAnnotationType(
                            GUID guid,
                            ABI::Windows::UI::UIAutomation::Core::AutomationAnnotationTypeRegistration* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnregisterAnnotationType(
                            ABI::Windows::UI::UIAutomation::Core::AutomationAnnotationTypeRegistration registration
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreAutomationRegistrarStatics = __uuidof(ICoreAutomationRegistrarStatics);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRemoteOperation[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("3ac656f4-e2bc-5c6e-b8e7-b224fb74b060")
                    ICoreAutomationRemoteOperation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsOpcodeSupported(
                            UINT32 opcode,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportElement(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId operandId,
                            ABI::Windows::UI::UIAutomation::IAutomationElement* element
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportTextRange(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId operandId,
                            ABI::Windows::UI::UIAutomation::IAutomationTextRange* textRange
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddToResults(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId operandId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Execute(
                            UINT32 bytecodeBufferLength,
                            BYTE* bytecodeBuffer,
                            ABI::Windows::UI::UIAutomation::Core::IAutomationRemoteOperationResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreAutomationRemoteOperation = __uuidof(ICoreAutomationRemoteOperation);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation2
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRemoteOperation2[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("eefaf86f-e953-5099-8ce9-dca813482ba0")
                    ICoreAutomationRemoteOperation2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ImportConnectionBoundObject(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId operandId,
                            ABI::Windows::UI::UIAutomation::IAutomationConnectionBoundObject* connectionBoundObject
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreAutomationRemoteOperation2 = __uuidof(ICoreAutomationRemoteOperation2);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationContext
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRemoteOperationContext[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationContext";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("b9af9cbb-3d3e-5918-a16b-7861626a3aeb")
                    ICoreAutomationRemoteOperationContext : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetOperand(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId id,
                            IInspectable** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetOperand(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId id,
                            IInspectable* operand
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetOperand2(
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId id,
                            IInspectable* operand,
                            GUID operandInterfaceId
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreAutomationRemoteOperationContext = __uuidof(ICoreAutomationRemoteOperationContext);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRemoteOperationExtensionProvider[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("88f53e67-dc69-553b-a0aa-70477e724da8")
                    ICoreAutomationRemoteOperationExtensionProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CallExtension(
                            GUID extensionId,
                            ABI::Windows::UI::UIAutomation::Core::ICoreAutomationRemoteOperationContext* context,
                            UINT32 operandIdsLength,
                            ABI::Windows::UI::UIAutomation::Core::AutomationRemoteOperationOperandId* operandIds
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsExtensionSupported(
                            GUID extensionId,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreAutomationRemoteOperationExtensionProvider = __uuidof(ICoreAutomationRemoteOperationExtensionProvider);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationClientSession
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationClientSession
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationClientSession[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationClientSession";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("5c8a091d-94cc-5b33-afdb-678cded2bd54")
                    IRemoteAutomationClientSession : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWindowAsync(
                            UINT64 remoteWindowId,
                            UINT32 remoteProcessId,
                            IInspectable* parentAutomationElement,
                            __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ConnectionRequested(
                            __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ConnectionRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Disconnected(
                            __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Disconnected(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteAutomationClientSession = __uuidof(IRemoteAutomationClientSession);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationClientSessionFactory
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationClientSession
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationClientSessionFactory[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationClientSessionFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("f250263d-6057-5373-a5a5-ed7265fe0376")
                    IRemoteAutomationClientSessionFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            HSTRING name,
                            ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationClientSession** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance2(
                            HSTRING name,
                            GUID sessionId,
                            ABI::Windows::UI::UIAutomation::Core::IRemoteAutomationClientSession** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteAutomationClientSessionFactory = __uuidof(IRemoteAutomationClientSessionFactory);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationConnectionRequestedEventArgs
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationConnectionRequestedEventArgs[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationConnectionRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("ea3319a8-e3a8-5dc6-adf8-044e46b14af5")
                    IRemoteAutomationConnectionRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LocalPipeName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RemoteProcessId(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteAutomationConnectionRequestedEventArgs = __uuidof(IRemoteAutomationConnectionRequestedEventArgs);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationDisconnectedEventArgs
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationDisconnectedEventArgs[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationDisconnectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("bbb33a3d-5d90-5c38-9eb2-dd9dcc1b2e3f")
                    IRemoteAutomationDisconnectedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LocalPipeName(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteAutomationDisconnectedEventArgs = __uuidof(IRemoteAutomationDisconnectedEventArgs);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationServerStatics
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationServer
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationServerStatics[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationServerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("e6e8945e-0c11-5028-9ae3-c2771288b6b7")
                    IRemoteAutomationServerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportSession(
                            GUID sessionId
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteAutomationServerStatics = __uuidof(IRemoteAutomationServerStatics);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationWindow
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationWindow
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationWindow[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationWindow";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace UIAutomation {
                namespace Core {
                    MIDL_INTERFACE("7c607689-496d-512a-9bd5-c050cfaf1428")
                    IRemoteAutomationWindow : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AutomationProvider(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnregisterAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteAutomationWindow = __uuidof(IRemoteAutomationWindow);
                } /* Core */
            } /* UIAutomation */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IAutomationRemoteOperationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_AutomationRemoteOperationResult_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_AutomationRemoteOperationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_AutomationRemoteOperationResult[] = L"Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIAutomation.Core.CoreAutomationRegistrar
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.UIAutomation.Core.ICoreAutomationRegistrarStatics interface starting with version 2.0 of the Windows.UI.UIAutomation.UIAutomationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRegistrar_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRegistrar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_CoreAutomationRegistrar[] = L"Windows.UI.UIAutomation.Core.CoreAutomationRegistrar";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.UIAutomation.UIAutomationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation ** Default Interface **
 *    Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperation_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperation[] = L"Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationContext ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperationContext_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperationContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperationContext[] = L"Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationClientSession
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.UIAutomation.Core.IRemoteAutomationClientSessionFactory interface starting with version 2.0 of the Windows.UI.UIAutomation.UIAutomationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IRemoteAutomationClientSession ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationClientSession_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationClientSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationClientSession[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationClientSession";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IRemoteAutomationConnectionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationConnectionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationConnectionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationConnectionRequestedEventArgs[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IRemoteAutomationDisconnectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationDisconnectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationDisconnectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationDisconnectedEventArgs[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationServer
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.UIAutomation.Core.IRemoteAutomationServerStatics interface starting with version 2.0 of the Windows.UI.UIAutomation.UIAutomationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationServer_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationServer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationServer[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationServer";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationWindow
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IRemoteAutomationWindow ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationWindow_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationWindow_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationWindow[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationWindow";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2 __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow;

#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow;

typedef struct __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindowVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_INTERFACE_DEFINED__
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* This,
        __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindowVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow_INTERFACE_DEFINED__
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* sender,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs* This,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* sender,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange;

#endif // ____x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationStatus __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationStatus;

typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationAnnotationTypeRegistration __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationAnnotationTypeRegistration;

typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId;

/*
 *
 * Struct Windows.UI.UIAutomation.Core.AutomationRemoteOperationStatus
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationStatus
{
    AutomationRemoteOperationStatus_Success = 0,
    AutomationRemoteOperationStatus_MalformedBytecode = 1,
    AutomationRemoteOperationStatus_InstructionLimitExceeded = 2,
    AutomationRemoteOperationStatus_UnhandledException = 3,
    AutomationRemoteOperationStatus_ExecutionFailure = 4,
};
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.UIAutomation.Core.AutomationAnnotationTypeRegistration
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationAnnotationTypeRegistration
{
    INT32 LocalId;
};
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.UI.UIAutomation.Core.AutomationRemoteOperationOperandId
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId
{
    INT32 Value;
};
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IAutomationRemoteOperationResult
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IAutomationRemoteOperationResult[] = L"Windows.UI.UIAutomation.Core.IAutomationRemoteOperationResult";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        enum __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorLocation)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* HasOperand)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId operandId,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetOperand)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId operandId,
        IInspectable** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResultVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_get_ErrorLocation(This, value) \
    ((This)->lpVtbl->get_ErrorLocation(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_HasOperand(This, operandId, result) \
    ((This)->lpVtbl->HasOperand(This, operandId, result))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_GetOperand(This, operandId, result) \
    ((This)->lpVtbl->GetOperand(This, operandId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationConnectionBoundObjectProvider[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsComThreadingRequired)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProviderVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_get_IsComThreadingRequired(This, value) \
    ((This)->lpVtbl->get_IsComThreadingRequired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationConnectionBoundObjectProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRegistrarStatics
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.CoreAutomationRegistrar
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRegistrarStatics[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRegistrarStatics";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RegisterAnnotationType)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics* This,
        GUID guid,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationAnnotationTypeRegistration* result);
    HRESULT (STDMETHODCALLTYPE* UnregisterAnnotationType)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationAnnotationTypeRegistration registration);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStaticsVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_RegisterAnnotationType(This, guid, result) \
    ((This)->lpVtbl->RegisterAnnotationType(This, guid, result))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_UnregisterAnnotationType(This, registration) \
    ((This)->lpVtbl->UnregisterAnnotationType(This, registration))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRegistrarStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRemoteOperation[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsOpcodeSupported)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        UINT32 opcode,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* ImportElement)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId operandId,
        __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationElement* element);
    HRESULT (STDMETHODCALLTYPE* ImportTextRange)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId operandId,
        __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationTextRange* textRange);
    HRESULT (STDMETHODCALLTYPE* AddToResults)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId operandId);
    HRESULT (STDMETHODCALLTYPE* Execute)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation* This,
        UINT32 bytecodeBufferLength,
        BYTE* bytecodeBuffer,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIAutomationRemoteOperationResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_IsOpcodeSupported(This, opcode, result) \
    ((This)->lpVtbl->IsOpcodeSupported(This, opcode, result))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_ImportElement(This, operandId, element) \
    ((This)->lpVtbl->ImportElement(This, operandId, element))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_ImportTextRange(This, operandId, textRange) \
    ((This)->lpVtbl->ImportTextRange(This, operandId, textRange))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_AddToResults(This, operandId) \
    ((This)->lpVtbl->AddToResults(This, operandId))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_Execute(This, bytecodeBufferLength, bytecodeBuffer, result) \
    ((This)->lpVtbl->Execute(This, bytecodeBufferLength, bytecodeBuffer, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation2
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRemoteOperation2[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation2";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ImportConnectionBoundObject)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId operandId,
        __x_ABI_CWindows_CUI_CUIAutomation_CIAutomationConnectionBoundObject* connectionBoundObject);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2Vtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_ImportConnectionBoundObject(This, operandId, connectionBoundObject) \
    ((This)->lpVtbl->ImportConnectionBoundObject(This, operandId, connectionBoundObject))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationContext
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRemoteOperationContext[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationContext";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetOperand)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId id,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* SetOperand)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId id,
        IInspectable* operand);
    HRESULT (STDMETHODCALLTYPE* SetOperand2)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* This,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId id,
        IInspectable* operand,
        GUID operandInterfaceId);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContextVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_GetOperand(This, id, result) \
    ((This)->lpVtbl->GetOperand(This, id, result))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_SetOperand(This, id, operand) \
    ((This)->lpVtbl->SetOperand(This, id, operand))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_SetOperand2(This, id, operand, operandInterfaceId) \
    ((This)->lpVtbl->SetOperand2(This, id, operand, operandInterfaceId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_ICoreAutomationRemoteOperationExtensionProvider[] = L"Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CallExtension)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider* This,
        GUID extensionId,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationContext* context,
        UINT32 operandIdsLength,
        struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CAutomationRemoteOperationOperandId* operandIds);
    HRESULT (STDMETHODCALLTYPE* IsExtensionSupported)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider* This,
        GUID extensionId,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProviderVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_CallExtension(This, extensionId, context, operandIdsLength, operandIds) \
    ((This)->lpVtbl->CallExtension(This, extensionId, context, operandIdsLength, operandIds))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_IsExtensionSupported(This, extensionId, result) \
    ((This)->lpVtbl->IsExtensionSupported(This, extensionId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CICoreAutomationRemoteOperationExtensionProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationClientSession
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationClientSession
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationClientSession[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationClientSession";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This);
    HRESULT (STDMETHODCALLTYPE* CreateWindowAsync)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        UINT64 remoteWindowId,
        UINT32 remoteProcessId,
        IInspectable* parentAutomationElement,
        __FIAsyncOperation_1_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationWindow** operation);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* add_ConnectionRequested)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationConnectionRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ConnectionRequested)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Disconnected)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        __FITypedEventHandler_2_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationClientSession_Windows__CUI__CUIAutomation__CCore__CRemoteAutomationDisconnectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Disconnected)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_CreateWindowAsync(This, remoteWindowId, remoteProcessId, parentAutomationElement, operation) \
    ((This)->lpVtbl->CreateWindowAsync(This, remoteWindowId, remoteProcessId, parentAutomationElement, operation))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_add_ConnectionRequested(This, handler, token) \
    ((This)->lpVtbl->add_ConnectionRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_remove_ConnectionRequested(This, token) \
    ((This)->lpVtbl->remove_ConnectionRequested(This, token))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_add_Disconnected(This, handler, token) \
    ((This)->lpVtbl->add_Disconnected(This, handler, token))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_remove_Disconnected(This, token) \
    ((This)->lpVtbl->remove_Disconnected(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationClientSessionFactory
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationClientSession
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationClientSessionFactory[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationClientSessionFactory";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstance2)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory* This,
        HSTRING name,
        GUID sessionId,
        __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSession** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactoryVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_CreateInstance(This, name, value) \
    ((This)->lpVtbl->CreateInstance(This, name, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_CreateInstance2(This, name, sessionId, value) \
    ((This)->lpVtbl->CreateInstance2(This, name, sessionId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationClientSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationConnectionRequestedEventArgs
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationConnectionRequestedEventArgs[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationConnectionRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalPipeName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteProcessId)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_get_LocalPipeName(This, value) \
    ((This)->lpVtbl->get_LocalPipeName(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_get_RemoteProcessId(This, value) \
    ((This)->lpVtbl->get_RemoteProcessId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationConnectionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationDisconnectedEventArgs
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationDisconnectedEventArgs[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationDisconnectedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalPipeName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_get_LocalPipeName(This, value) \
    ((This)->lpVtbl->get_LocalPipeName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationDisconnectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationServerStatics
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationServer
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationServerStatics[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationServerStatics";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportSession)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics* This,
        GUID sessionId);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_ReportSession(This, sessionId) \
    ((This)->lpVtbl->ReportSession(This, sessionId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationServerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.UIAutomation.Core.IRemoteAutomationWindow
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIAutomation.Core.RemoteAutomationWindow
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_UIAutomation_Core_IRemoteAutomationWindow[] = L"Windows.UI.UIAutomation.Core.IRemoteAutomationWindow";
typedef struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutomationProvider)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* UnregisterAsync)(__x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindowVtbl;

interface __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_get_AutomationProvider(This, value) \
    ((This)->lpVtbl->get_AutomationProvider(This, value))

#define __x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_UnregisterAsync(This, operation) \
    ((This)->lpVtbl->UnregisterAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow;
#endif /* !defined(____x_ABI_CWindows_CUI_CUIAutomation_CCore_CIRemoteAutomationWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IAutomationRemoteOperationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_AutomationRemoteOperationResult_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_AutomationRemoteOperationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_AutomationRemoteOperationResult[] = L"Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIAutomation.Core.CoreAutomationRegistrar
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.UIAutomation.Core.ICoreAutomationRegistrarStatics interface starting with version 2.0 of the Windows.UI.UIAutomation.UIAutomationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRegistrar_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRegistrar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_CoreAutomationRegistrar[] = L"Windows.UI.UIAutomation.Core.CoreAutomationRegistrar";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.UIAutomation.UIAutomationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation ** Default Interface **
 *    Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperation_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperation[] = L"Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationContext ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperationContext_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperationContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_CoreAutomationRemoteOperationContext[] = L"Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationClientSession
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.UIAutomation.Core.IRemoteAutomationClientSessionFactory interface starting with version 2.0 of the Windows.UI.UIAutomation.UIAutomationContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IRemoteAutomationClientSession ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationClientSession_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationClientSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationClientSession[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationClientSession";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IRemoteAutomationConnectionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationConnectionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationConnectionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationConnectionRequestedEventArgs[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IRemoteAutomationDisconnectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationDisconnectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationDisconnectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationDisconnectedEventArgs[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationServer
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.UIAutomation.Core.IRemoteAutomationServerStatics interface starting with version 2.0 of the Windows.UI.UIAutomation.UIAutomationContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationServer_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationServer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationServer[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationServer";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.UIAutomation.Core.RemoteAutomationWindow
 *
 * Introduced to Windows.UI.UIAutomation.UIAutomationContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.UIAutomation.Core.IRemoteAutomationWindow ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationWindow_DEFINED
#define RUNTIMECLASS_Windows_UI_UIAutomation_Core_RemoteAutomationWindow_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIAutomation_Core_RemoteAutomationWindow[] = L"Windows.UI.UIAutomation.Core.RemoteAutomationWindow";
#endif
#endif // WINDOWS_UI_UIAUTOMATION_UIAUTOMATIONCONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Euiautomation2Ecore_p_h__

#endif // __windows2Eui2Euiautomation2Ecore_h__
