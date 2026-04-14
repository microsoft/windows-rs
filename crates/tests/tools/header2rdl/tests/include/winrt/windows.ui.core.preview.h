
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
#ifndef __windows2Eui2Ecore2Epreview_h__
#define __windows2Eui2Ecore2Epreview_h__
#ifndef __windows2Eui2Ecore2Epreview_p_h__
#define __windows2Eui2Ecore2Epreview_p_h__


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
#include "Windows.UI.WindowManagement.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    interface ICoreAppWindowPreview;
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview ABI::Windows::UI::Core::Preview::ICoreAppWindowPreview

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    interface ICoreAppWindowPreviewStatics;
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics ABI::Windows::UI::Core::Preview::ICoreAppWindowPreviewStatics

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    interface ISystemNavigationCloseRequestedPreviewEventArgs;
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs ABI::Windows::UI::Core::Preview::ISystemNavigationCloseRequestedPreviewEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    interface ISystemNavigationManagerPreview;
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview ABI::Windows::UI::Core::Preview::ISystemNavigationManagerPreview

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    interface ISystemNavigationManagerPreviewStatics;
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics ABI::Windows::UI::Core::Preview::ISystemNavigationManagerPreviewStatics

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    class SystemNavigationCloseRequestedPreviewEventArgs;
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("01bca043-4d09-59e4-b1b3-a2ce24629e41"))
IEventHandler<ABI::Windows::UI::Core::Preview::SystemNavigationCloseRequestedPreviewEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::Preview::SystemNavigationCloseRequestedPreviewEventArgs*, ABI::Windows::UI::Core::Preview::ISystemNavigationCloseRequestedPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::UI::Core::Preview::SystemNavigationCloseRequestedPreviewEventArgs*> __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_t;
#define __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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
        namespace UI {
            namespace WindowManagement {
                class AppWindow;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindow;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow ABI::Windows::UI::WindowManagement::IAppWindow

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    class SystemNavigationManagerPreview;
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.UI.Core.Preview.ICoreAppWindowPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.CoreAppWindowPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ICoreAppWindowPreview[] = L"Windows.UI.Core.Preview.ICoreAppWindowPreview";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    MIDL_INTERFACE("a4f6e665-365e-5fde-87a5-9543c3a15aa8")
                    ICoreAppWindowPreview : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ICoreAppWindowPreview = __uuidof(ICoreAppWindowPreview);
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.CoreAppWindowPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ICoreAppWindowPreviewStatics[] = L"Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    MIDL_INTERFACE("33ac21be-423b-5db6-8a8e-4dc87353b75b")
                    ICoreAppWindowPreviewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetIdFromWindow(
                            ABI::Windows::UI::WindowManagement::IAppWindow* window,
                            INT32* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreAppWindowPreviewStatics = __uuidof(ICoreAppWindowPreviewStatics);
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ISystemNavigationCloseRequestedPreviewEventArgs[] = L"Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    MIDL_INTERFACE("83d00de1-cbe5-4f31-8414-361da046518f")
                    ISystemNavigationCloseRequestedPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISystemNavigationCloseRequestedPreviewEventArgs = __uuidof(ISystemNavigationCloseRequestedPreviewEventArgs);
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Core.Preview.ISystemNavigationManagerPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.SystemNavigationManagerPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ISystemNavigationManagerPreview[] = L"Windows.UI.Core.Preview.ISystemNavigationManagerPreview";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    MIDL_INTERFACE("ec5f0488-6425-4777-a536-cb5634427f0d")
                    ISystemNavigationManagerPreview : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_CloseRequested(
                            __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CloseRequested(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISystemNavigationManagerPreview = __uuidof(ISystemNavigationManagerPreview);
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Core.Preview.ISystemNavigationManagerPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.SystemNavigationManagerPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ISystemNavigationManagerPreviewStatics[] = L"Windows.UI.Core.Preview.ISystemNavigationManagerPreviewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                namespace Preview {
                    MIDL_INTERFACE("0e971360-df74-4bce-84cb-bd1181ac0a71")
                    ISystemNavigationManagerPreviewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::UI::Core::Preview::ISystemNavigationManagerPreview** loader
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISystemNavigationManagerPreviewStatics = __uuidof(ISystemNavigationManagerPreviewStatics);
                } /* Preview */
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Core.Preview.CoreAppWindowPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.Preview.ICoreAppWindowPreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Core_Preview_CoreAppWindowPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_Preview_CoreAppWindowPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_Preview_CoreAppWindowPreview[] = L"Windows.UI.Core.Preview.CoreAppWindowPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Core_Preview_SystemNavigationCloseRequestedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_Preview_SystemNavigationCloseRequestedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_Preview_SystemNavigationCloseRequestedPreviewEventArgs[] = L"Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Core.Preview.SystemNavigationManagerPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.Preview.ISystemNavigationManagerPreviewStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.Preview.ISystemNavigationManagerPreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Core_Preview_SystemNavigationManagerPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_Preview_SystemNavigationManagerPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_Preview_SystemNavigationManagerPreview[] = L"Windows.UI.Core.Preview.SystemNavigationManagerPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview;

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics;

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview;

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics;

#endif // ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs;

typedef struct __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgsVtbl;

interface __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__

/*
 *
 * Interface Windows.UI.Core.Preview.ICoreAppWindowPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.CoreAppWindowPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ICoreAppWindowPreview[] = L"Windows.UI.Core.Preview.ICoreAppWindowPreview";
typedef struct __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewVtbl;

interface __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.CoreAppWindowPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ICoreAppWindowPreviewStatics[] = L"Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics";
typedef struct __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetIdFromWindow)(__x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* window,
        INT32* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_GetIdFromWindow(This, window, result) \
    ((This)->lpVtbl->GetIdFromWindow(This, window, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CICoreAppWindowPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ISystemNavigationCloseRequestedPreviewEventArgs[] = L"Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationCloseRequestedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Core.Preview.ISystemNavigationManagerPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.SystemNavigationManagerPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ISystemNavigationManagerPreview[] = L"Windows.UI.Core.Preview.ISystemNavigationManagerPreview";
typedef struct __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_CloseRequested)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview* This,
        __FIEventHandler_1_Windows__CUI__CCore__CPreview__CSystemNavigationCloseRequestedPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CloseRequested)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewVtbl;

interface __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_add_CloseRequested(This, handler, token) \
    ((This)->lpVtbl->add_CloseRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_remove_CloseRequested(This, token) \
    ((This)->lpVtbl->remove_CloseRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Core.Preview.ISystemNavigationManagerPreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.Preview.SystemNavigationManagerPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_Preview_ISystemNavigationManagerPreviewStatics[] = L"Windows.UI.Core.Preview.ISystemNavigationManagerPreviewStatics";
typedef struct __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics* This,
        __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreview** loader);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_GetForCurrentView(This, loader) \
    ((This)->lpVtbl->GetForCurrentView(This, loader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CPreview_CISystemNavigationManagerPreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Core.Preview.CoreAppWindowPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.Preview.ICoreAppWindowPreviewStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.Preview.ICoreAppWindowPreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_Core_Preview_CoreAppWindowPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_Preview_CoreAppWindowPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_Preview_CoreAppWindowPreview[] = L"Windows.UI.Core.Preview.CoreAppWindowPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.Preview.ISystemNavigationCloseRequestedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Core_Preview_SystemNavigationCloseRequestedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_Preview_SystemNavigationCloseRequestedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_Preview_SystemNavigationCloseRequestedPreviewEventArgs[] = L"Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Core.Preview.SystemNavigationManagerPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.Preview.ISystemNavigationManagerPreviewStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.Preview.ISystemNavigationManagerPreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Core_Preview_SystemNavigationManagerPreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_Preview_SystemNavigationManagerPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_Preview_SystemNavigationManagerPreview[] = L"Windows.UI.Core.Preview.SystemNavigationManagerPreview";
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
#endif // __windows2Eui2Ecore2Epreview_p_h__

#endif // __windows2Eui2Ecore2Epreview_h__
