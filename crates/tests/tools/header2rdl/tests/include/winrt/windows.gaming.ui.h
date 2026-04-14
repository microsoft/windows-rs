
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
#ifndef __windows2Egaming2Eui_h__
#define __windows2Egaming2Eui_h__
#ifndef __windows2Egaming2Eui_p_h__
#define __windows2Egaming2Eui_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION)
#define WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION)

#if !defined(WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION)
#define WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.Activation.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                interface IGameBarStatics;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics ABI::Windows::Gaming::UI::IGameBarStatics

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                interface IGameChatMessageReceivedEventArgs;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs ABI::Windows::Gaming::UI::IGameChatMessageReceivedEventArgs

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                interface IGameChatOverlay;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay ABI::Windows::Gaming::UI::IGameChatOverlay

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                interface IGameChatOverlayMessageSource;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource ABI::Windows::Gaming::UI::IGameChatOverlayMessageSource

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                interface IGameChatOverlayStatics;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics ABI::Windows::Gaming::UI::IGameChatOverlayStatics

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                interface IGameUIProviderActivatedEventArgs;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs ABI::Windows::Gaming::UI::IGameUIProviderActivatedEventArgs

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                class GameChatOverlayMessageSource;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                class GameChatMessageReceivedEventArgs;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fe4f13bf-689c-5fe3-b7ad-55bc57f92466"))
ITypedEventHandler<ABI::Windows::Gaming::UI::GameChatOverlayMessageSource*, ABI::Windows::Gaming::UI::GameChatMessageReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::UI::GameChatOverlayMessageSource*, ABI::Windows::Gaming::UI::IGameChatOverlayMessageSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::UI::GameChatMessageReceivedEventArgs*, ABI::Windows::Gaming::UI::IGameChatMessageReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Gaming.UI.GameChatOverlayMessageSource, Windows.Gaming.UI.GameChatMessageReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Gaming::UI::GameChatOverlayMessageSource*, ABI::Windows::Gaming::UI::GameChatMessageReceivedEventArgs*> __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_USE */

#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class ValueSet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                typedef enum GameChatMessageOrigin : int GameChatMessageOrigin;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                typedef enum GameChatOverlayPosition : int GameChatOverlayPosition;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                class GameChatOverlay;
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Gaming.UI.GameChatMessageOrigin
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                enum GameChatMessageOrigin : int
                {
                    GameChatMessageOrigin_Voice = 0,
                    GameChatMessageOrigin_Text = 1,
                };
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.UI.GameChatOverlayPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                enum GameChatOverlayPosition : int
                {
                    GameChatOverlayPosition_BottomCenter = 0,
                    GameChatOverlayPosition_BottomLeft = 1,
                    GameChatOverlayPosition_BottomRight = 2,
                    GameChatOverlayPosition_MiddleRight = 3,
                    GameChatOverlayPosition_MiddleLeft = 4,
                    GameChatOverlayPosition_TopCenter = 5,
                    GameChatOverlayPosition_TopLeft = 6,
                    GameChatOverlayPosition_TopRight = 7,
                };
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.UI.IGameBarStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameBarStatics[] = L"Windows.Gaming.UI.IGameBarStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                MIDL_INTERFACE("1db9a292-cc78-4173-be45-b61e67283ea7")
                IGameBarStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_VisibilityChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_VisibilityChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsInputRedirectedChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsInputRedirectedChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Visible(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInputRedirected(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGameBarStatics = __uuidof(IGameBarStatics);
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameBarStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.UI.IGameChatMessageReceivedEventArgs
 *
 * Introduced to Windows.Gaming.UI.GameChatOverlayContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameChatMessageReceivedEventArgs
 *
 */
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameChatMessageReceivedEventArgs[] = L"Windows.Gaming.UI.IGameChatMessageReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                MIDL_INTERFACE("a28201f1-3fb9-4e42-a403-7afce2023b1e")
                IGameChatMessageReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppDisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SenderName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Origin(
                        ABI::Windows::Gaming::UI::GameChatMessageOrigin* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGameChatMessageReceivedEventArgs = __uuidof(IGameChatMessageReceivedEventArgs);
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.UI.IGameChatOverlay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameChatOverlay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameChatOverlay[] = L"Windows.Gaming.UI.IGameChatOverlay";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                MIDL_INTERFACE("fbc64865-f6fc-4a48-ae07-03ac6ed43704")
                IGameChatOverlay : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredPosition(
                        ABI::Windows::Gaming::UI::GameChatOverlayPosition* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredPosition(
                        ABI::Windows::Gaming::UI::GameChatOverlayPosition value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddMessage(
                        HSTRING sender,
                        HSTRING message,
                        ABI::Windows::Gaming::UI::GameChatMessageOrigin origin
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGameChatOverlay = __uuidof(IGameChatOverlay);
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.UI.IGameChatOverlayMessageSource
 *
 * Introduced to Windows.Gaming.UI.GameChatOverlayContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameChatOverlayMessageSource
 *
 */
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameChatOverlayMessageSource[] = L"Windows.Gaming.UI.IGameChatOverlayMessageSource";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                MIDL_INTERFACE("1e177397-59fb-4f4f-8e9a-80acf817743c")
                IGameChatOverlayMessageSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_MessageReceived(
                        __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MessageReceived(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDelayBeforeClosingAfterMessageReceived(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGameChatOverlayMessageSource = __uuidof(IGameChatOverlayMessageSource);
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.UI.IGameChatOverlayStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameChatOverlay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameChatOverlayStatics[] = L"Windows.Gaming.UI.IGameChatOverlayStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                MIDL_INTERFACE("89acf614-7867-49f7-9687-25d9dbf444d1")
                IGameChatOverlayStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Gaming::UI::IGameChatOverlay** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGameChatOverlayStatics = __uuidof(IGameChatOverlayStatics);
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.UI.IGameUIProviderActivatedEventArgs
 *
 * Introduced to Windows.Gaming.UI.GamingUIProviderContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameUIProviderActivatedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameUIProviderActivatedEventArgs[] = L"Windows.Gaming.UI.IGameUIProviderActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace UI {
                MIDL_INTERFACE("a7b3203e-caf7-4ded-bbd2-47de43bb6dd5")
                IGameUIProviderActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_GameUIArgs(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportCompleted(
                        ABI::Windows::Foundation::Collections::IPropertySet* results
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGameUIProviderActivatedEventArgs = __uuidof(IGameUIProviderActivatedEventArgs);
            } /* UI */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.UI.GameBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.UI.IGameBarStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameBar_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameBar[] = L"Windows.Gaming.UI.GameBar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Gaming.UI.GameChatMessageReceivedEventArgs
 *
 * Introduced to Windows.Gaming.UI.GameChatOverlayContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.UI.IGameChatMessageReceivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameChatMessageReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameChatMessageReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameChatMessageReceivedEventArgs[] = L"Windows.Gaming.UI.GameChatMessageReceivedEventArgs";
#endif
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.UI.GameChatOverlay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.UI.IGameChatOverlayStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.UI.IGameChatOverlay ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameChatOverlay_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameChatOverlay_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameChatOverlay[] = L"Windows.Gaming.UI.GameChatOverlay";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.UI.GameChatOverlayMessageSource
 *
 * Introduced to Windows.Gaming.UI.GameChatOverlayContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Gaming.UI.GameChatOverlayContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.UI.IGameChatOverlayMessageSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameChatOverlayMessageSource_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameChatOverlayMessageSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameChatOverlayMessageSource[] = L"Windows.Gaming.UI.GameChatOverlayMessageSource";
#endif
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.UI.GameUIProviderActivatedEventArgs
 *
 * Introduced to Windows.Gaming.UI.GamingUIProviderContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.UI.IGameUIProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameUIProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameUIProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameUIProviderActivatedEventArgs[] = L"Windows.Gaming.UI.GameUIProviderActivatedEventArgs";
#endif
#endif // WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics;

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs;

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay;

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource;

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics;

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs;

#endif // ____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* sender,
        __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CGaming_CUI_CGameChatMessageOrigin __x_ABI_CWindows_CGaming_CUI_CGameChatMessageOrigin;

typedef enum __x_ABI_CWindows_CGaming_CUI_CGameChatOverlayPosition __x_ABI_CWindows_CGaming_CUI_CGameChatOverlayPosition;

/*
 *
 * Struct Windows.Gaming.UI.GameChatMessageOrigin
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGaming_CUI_CGameChatMessageOrigin
{
    GameChatMessageOrigin_Voice = 0,
    GameChatMessageOrigin_Text = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Gaming.UI.GameChatOverlayPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGaming_CUI_CGameChatOverlayPosition
{
    GameChatOverlayPosition_BottomCenter = 0,
    GameChatOverlayPosition_BottomLeft = 1,
    GameChatOverlayPosition_BottomRight = 2,
    GameChatOverlayPosition_MiddleRight = 3,
    GameChatOverlayPosition_MiddleLeft = 4,
    GameChatOverlayPosition_TopCenter = 5,
    GameChatOverlayPosition_TopLeft = 6,
    GameChatOverlayPosition_TopRight = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.UI.IGameBarStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameBarStatics[] = L"Windows.Gaming.UI.IGameBarStatics";
typedef struct __x_ABI_CWindows_CGaming_CUI_CIGameBarStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_VisibilityChanged)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_VisibilityChanged)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_IsInputRedirectedChanged)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsInputRedirectedChanged)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Visible)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInputRedirected)(__x_ABI_CWindows_CGaming_CUI_CIGameBarStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CUI_CIGameBarStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CUI_CIGameBarStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_add_VisibilityChanged(This, handler, token) \
    ((This)->lpVtbl->add_VisibilityChanged(This, handler, token))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_remove_VisibilityChanged(This, token) \
    ((This)->lpVtbl->remove_VisibilityChanged(This, token))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_add_IsInputRedirectedChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsInputRedirectedChanged(This, handler, token))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_remove_IsInputRedirectedChanged(This, token) \
    ((This)->lpVtbl->remove_IsInputRedirectedChanged(This, token))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_get_Visible(This, value) \
    ((This)->lpVtbl->get_Visible(This, value))

#define __x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_get_IsInputRedirected(This, value) \
    ((This)->lpVtbl->get_IsInputRedirected(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameBarStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameBarStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.UI.IGameChatMessageReceivedEventArgs
 *
 * Introduced to Windows.Gaming.UI.GameChatOverlayContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameChatMessageReceivedEventArgs
 *
 */
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameChatMessageReceivedEventArgs[] = L"Windows.Gaming.UI.IGameChatMessageReceivedEventArgs";
typedef struct __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppId)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppDisplayName)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SenderName)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Origin)(__x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs* This,
        enum __x_ABI_CWindows_CGaming_CUI_CGameChatMessageOrigin* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_get_AppId(This, value) \
    ((This)->lpVtbl->get_AppId(This, value))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_get_AppDisplayName(This, value) \
    ((This)->lpVtbl->get_AppDisplayName(This, value))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_get_SenderName(This, value) \
    ((This)->lpVtbl->get_SenderName(This, value))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_get_Origin(This, value) \
    ((This)->lpVtbl->get_Origin(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatMessageReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.UI.IGameChatOverlay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameChatOverlay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameChatOverlay[] = L"Windows.Gaming.UI.IGameChatOverlay";
typedef struct __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesiredPosition)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This,
        enum __x_ABI_CWindows_CGaming_CUI_CGameChatOverlayPosition* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredPosition)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This,
        enum __x_ABI_CWindows_CGaming_CUI_CGameChatOverlayPosition value);
    HRESULT (STDMETHODCALLTYPE* AddMessage)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay* This,
        HSTRING sender,
        HSTRING message,
        enum __x_ABI_CWindows_CGaming_CUI_CGameChatMessageOrigin origin);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayVtbl;

interface __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_get_DesiredPosition(This, value) \
    ((This)->lpVtbl->get_DesiredPosition(This, value))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_put_DesiredPosition(This, value) \
    ((This)->lpVtbl->put_DesiredPosition(This, value))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_AddMessage(This, sender, message, origin) \
    ((This)->lpVtbl->AddMessage(This, sender, message, origin))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.UI.IGameChatOverlayMessageSource
 *
 * Introduced to Windows.Gaming.UI.GameChatOverlayContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameChatOverlayMessageSource
 *
 */
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameChatOverlayMessageSource[] = L"Windows.Gaming.UI.IGameChatOverlayMessageSource";
typedef struct __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MessageReceived)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This,
        __FITypedEventHandler_2_Windows__CGaming__CUI__CGameChatOverlayMessageSource_Windows__CGaming__CUI__CGameChatMessageReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MessageReceived)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* SetDelayBeforeClosingAfterMessageReceived)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSourceVtbl;

interface __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_add_MessageReceived(This, handler, token) \
    ((This)->lpVtbl->add_MessageReceived(This, handler, token))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_remove_MessageReceived(This, token) \
    ((This)->lpVtbl->remove_MessageReceived(This, token))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_SetDelayBeforeClosingAfterMessageReceived(This, value) \
    ((This)->lpVtbl->SetDelayBeforeClosingAfterMessageReceived(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayMessageSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.UI.IGameChatOverlayStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameChatOverlay
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameChatOverlayStatics[] = L"Windows.Gaming.UI.IGameChatOverlayStatics";
typedef struct __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics* This,
        __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlay** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_GetDefault(This, value) \
    ((This)->lpVtbl->GetDefault(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameChatOverlayStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.UI.IGameUIProviderActivatedEventArgs
 *
 * Introduced to Windows.Gaming.UI.GamingUIProviderContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.UI.GameUIProviderActivatedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_UI_IGameUIProviderActivatedEventArgs[] = L"Windows.Gaming.UI.IGameUIProviderActivatedEventArgs";
typedef struct __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_GameUIArgs)(__x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompleted)(__x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* results);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_get_GameUIArgs(This, value) \
    ((This)->lpVtbl->get_GameUIArgs(This, value))

#define __x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_ReportCompleted(This, results) \
    ((This)->lpVtbl->ReportCompleted(This, results))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGaming_CUI_CIGameUIProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.UI.GameBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.UI.IGameBarStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameBar_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameBar[] = L"Windows.Gaming.UI.GameBar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Gaming.UI.GameChatMessageReceivedEventArgs
 *
 * Introduced to Windows.Gaming.UI.GameChatOverlayContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.UI.IGameChatMessageReceivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameChatMessageReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameChatMessageReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameChatMessageReceivedEventArgs[] = L"Windows.Gaming.UI.GameChatMessageReceivedEventArgs";
#endif
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.UI.GameChatOverlay
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.UI.IGameChatOverlayStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.UI.IGameChatOverlay ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameChatOverlay_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameChatOverlay_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameChatOverlay[] = L"Windows.Gaming.UI.GameChatOverlay";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.UI.GameChatOverlayMessageSource
 *
 * Introduced to Windows.Gaming.UI.GameChatOverlayContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Gaming.UI.GameChatOverlayContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.UI.IGameChatOverlayMessageSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameChatOverlayMessageSource_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameChatOverlayMessageSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameChatOverlayMessageSource[] = L"Windows.Gaming.UI.GameChatOverlayMessageSource";
#endif
#endif // WINDOWS_GAMING_UI_GAMECHATOVERLAYCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.UI.GameUIProviderActivatedEventArgs
 *
 * Introduced to Windows.Gaming.UI.GamingUIProviderContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.UI.IGameUIProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_UI_GameUIProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Gaming_UI_GameUIProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_UI_GameUIProviderActivatedEventArgs[] = L"Windows.Gaming.UI.GameUIProviderActivatedEventArgs";
#endif
#endif // WINDOWS_GAMING_UI_GAMINGUIPROVIDERCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egaming2Eui_p_h__

#endif // __windows2Egaming2Eui_h__
