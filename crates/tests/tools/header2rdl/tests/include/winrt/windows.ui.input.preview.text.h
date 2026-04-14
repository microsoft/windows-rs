
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
#ifndef __windows2Eui2Einput2Epreview2Etext_h__
#define __windows2Eui2Einput2Epreview2Etext_h__
#ifndef __windows2Eui2Einput2Epreview2Etext_p_h__
#define __windows2Eui2Einput2Epreview2Etext_p_h__


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

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#if !defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)

#if !defined(WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION)
#define WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.System.h"
#include "Windows.UI.h"
#include "Windows.UI.Core.h"
#include "Windows.UI.Text.h"
#include "Windows.UI.Text.Core.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface IConversionModeChangedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs ABI::Windows::UI::Input::Preview::Text::IConversionModeChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface IFocusEnteredEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs ABI::Windows::UI::Input::Preview::Text::IFocusEnteredEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface IInputDelegationModeChangedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs ABI::Windows::UI::Input::Preview::Text::IInputDelegationModeChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface IKeyEventReceivedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs ABI::Windows::UI::Input::Preview::Text::IKeyEventReceivedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface IKeyboardInputProcessor;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface IReconversionRequestedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs ABI::Windows::UI::Input::Preview::Text::IReconversionRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextBoxContentChangedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs ABI::Windows::UI::Input::Preview::Text::ITextBoxContentChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextBoxInfo;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo ABI::Windows::UI::Input::Preview::Text::ITextBoxInfo

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextBoxInfoChangedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs ABI::Windows::UI::Input::Preview::Text::ITextBoxInfoChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextComposition;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition ABI::Windows::UI::Input::Preview::Text::ITextComposition

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextCompositionSegment;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment ABI::Windows::UI::Input::Preview::Text::ITextCompositionSegment

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextEditSession;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession ABI::Windows::UI::Input::Preview::Text::ITextEditSession

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextInputProvider;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider ABI::Windows::UI::Input::Preview::Text::ITextInputProvider

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextInputService;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService ABI::Windows::UI::Input::Preview::Text::ITextInputService

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        interface ITextInputServiceStatics;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics ABI::Windows::UI::Input::Preview::Text::ITextInputServiceStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef enum PayloadResult : int PayloadResult;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f560216c-ced4-5ff5-ae23-4013064239c2"))
IAsyncOperation<enum ABI::Windows::UI::Input::Preview::Text::PayloadResult> : IAsyncOperation_impl<enum ABI::Windows::UI::Input::Preview::Text::PayloadResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Input.Preview.Text.PayloadResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::UI::Input::Preview::Text::PayloadResult> __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_t;
#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2a6f5a44-1662-54cc-aca5-1ce4346f0408"))
IAsyncOperationCompletedHandler<enum ABI::Windows::UI::Input::Preview::Text::PayloadResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::UI::Input::Preview::Text::PayloadResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Input.Preview.Text.PayloadResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::UI::Input::Preview::Text::PayloadResult> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CRect_USE
#define DEF___FIReference_1_Windows__CFoundation__CRect_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("80423f11-054f-5eac-afd3-63b6ce15e77b"))
IReference<struct ABI::Windows::Foundation::Rect> : IReference_impl<struct ABI::Windows::Foundation::Rect>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.Rect>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::Rect> __FIReference_1_Windows__CFoundation__CRect_t;
#define __FIReference_1_Windows__CFoundation__CRect ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CRect_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CRect_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class KeyboardInputProcessor;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f2171eb7-e800-51cf-a5bf-dbd9e384bca8"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.KeyboardInputProcessor, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class ConversionModeChangedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("af7aff76-8345-5ae5-a80a-92ee36b4a233"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::ConversionModeChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::ConversionModeChangedEventArgs*, ABI::Windows::UI::Input::Preview::Text::IConversionModeChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.KeyboardInputProcessor, Windows.UI.Input.Preview.Text.ConversionModeChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::ConversionModeChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class FocusEnteredEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50c0eea-7187-5086-ab7f-b4bcb6cce480"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::FocusEnteredEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::FocusEnteredEventArgs*, ABI::Windows::UI::Input::Preview::Text::IFocusEnteredEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.KeyboardInputProcessor, Windows.UI.Input.Preview.Text.FocusEnteredEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::FocusEnteredEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class KeyEventReceivedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ae1de89e-6bae-5995-94c2-a62e2fbdaf69"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::KeyEventReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::KeyEventReceivedEventArgs*, ABI::Windows::UI::Input::Preview::Text::IKeyEventReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.KeyboardInputProcessor, Windows.UI.Input.Preview.Text.KeyEventReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::KeyEventReceivedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class ReconversionRequestedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b4830ed2-e0fb-5c01-b469-4dfae2946aae"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::ReconversionRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::ReconversionRequestedEventArgs*, ABI::Windows::UI::Input::Preview::Text::IReconversionRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.KeyboardInputProcessor, Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::ReconversionRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class TextBoxContentChangedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("473fd59d-ca4d-5ce6-97c8-d5362596d1ca"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::TextBoxContentChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextBoxContentChangedEventArgs*, ABI::Windows::UI::Input::Preview::Text::ITextBoxContentChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.KeyboardInputProcessor, Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::TextBoxContentChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class TextBoxInfoChangedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ffba82d2-e603-5a55-b161-a5bd7b38b793"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::TextBoxInfoChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextBoxInfoChangedEventArgs*, ABI::Windows::UI::Input::Preview::Text::ITextBoxInfoChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.KeyboardInputProcessor, Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::KeyboardInputProcessor*, ABI::Windows::UI::Input::Preview::Text::TextBoxInfoChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class TextInputProvider;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b97b97e2-a03d-57f8-83e0-295b41fc772f"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::ITextInputProvider*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.TextInputProvider, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("da602ecb-4884-5c20-aaf0-6396e84141a4"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::FocusEnteredEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::ITextInputProvider*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::FocusEnteredEventArgs*, ABI::Windows::UI::Input::Preview::Text::IFocusEnteredEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.TextInputProvider, Windows.UI.Input.Preview.Text.FocusEnteredEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::FocusEnteredEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class InputDelegationModeChangedEventArgs;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("858b18cd-d7f5-52d9-93b5-154d4468d57e"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::InputDelegationModeChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::ITextInputProvider*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::InputDelegationModeChangedEventArgs*, ABI::Windows::UI::Input::Preview::Text::IInputDelegationModeChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.TextInputProvider, Windows.UI.Input.Preview.Text.InputDelegationModeChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::InputDelegationModeChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("89f1e494-9a28-5e03-82b3-cb54a6e8cfa8"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::ReconversionRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::ITextInputProvider*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::ReconversionRequestedEventArgs*, ABI::Windows::UI::Input::Preview::Text::IReconversionRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.TextInputProvider, Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::ReconversionRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e71f949e-b40e-5f14-9a37-8e4e3380b510"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::TextBoxContentChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::ITextInputProvider*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextBoxContentChangedEventArgs*, ABI::Windows::UI::Input::Preview::Text::ITextBoxContentChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.TextInputProvider, Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::TextBoxContentChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8eee49d4-5410-50ca-bc92-a63cefc9f510"))
ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::TextBoxInfoChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::ITextInputProvider*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Text::TextBoxInfoChangedEventArgs*, ABI::Windows::UI::Input::Preview::Text::ITextBoxInfoChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Preview.Text.TextInputProvider, Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Preview::Text::TextInputProvider*, ABI::Windows::UI::Input::Preview::Text::TextBoxInfoChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_USE */

#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IPropertyValue;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIPropertyValue ABI::Windows::Foundation::IPropertyValue

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            typedef enum VirtualKey : int VirtualKey;
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef struct CorePhysicalKeyStatus CorePhysicalKeyStatus;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    typedef enum CoreTextFormatUpdatingReason : int CoreTextFormatUpdatingReason;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    typedef enum CoreTextInputScope : int CoreTextInputScope;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    typedef struct CoreTextRange CoreTextRange;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                typedef enum UnderlineType : int UnderlineType;
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef enum KeyEventDeviceType : int KeyEventDeviceType;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef enum TextBoxContentAttribute : int TextBoxContentAttribute;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef enum TextBoxFeatures : unsigned int TextBoxFeatures;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef enum TextBoxSettings : unsigned int TextBoxSettings;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef enum TextChangeSource : int TextChangeSource;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef enum TextConversionMode : int TextConversionMode;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef enum TextStyleAttributes : unsigned int TextStyleAttributes;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef struct TextBoxId TextBoxId;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef struct TextInputServiceSubscription TextInputServiceSubscription;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        typedef struct TextStyle TextStyle;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class TextBoxInfo;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class TextComposition;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class TextCompositionSegment;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class TextEditSession;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        class TextInputService;
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Input.Preview.Text.KeyEventDeviceType
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        enum KeyEventDeviceType : int
                        {
                            KeyEventDeviceType_Undefined = 0,
                            KeyEventDeviceType_HardwareKeyboard = 1,
                            KeyEventDeviceType_SoftwareKeyboard = 2,
                            KeyEventDeviceType_Gamepad = 3,
                            KeyEventDeviceType_Injection = 4,
                        };
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.PayloadResult
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        enum PayloadResult : int
                        {
                            PayloadResult_InEditing = 0,
                            PayloadResult_Pending = 1,
                            PayloadResult_Completed = 2,
                            PayloadResult_Overridden = 3,
                            PayloadResult_Outrun = 4,
                            PayloadResult_Rejected = 5,
                            PayloadResult_Canceled = 6,
                        };
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextBoxContentAttribute
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        enum TextBoxContentAttribute : int
                        {
                            TextBoxContentAttribute_None = 0,
                            TextBoxContentAttribute_Selection = 1,
                            TextBoxContentAttribute_Text = 2,
                            TextBoxContentAttribute_Property = 3,
                            TextBoxContentAttribute_Layout = 4,
                        };
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextBoxFeatures
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        enum TextBoxFeatures : unsigned int
                        {
                            TextBoxFeatures_None = 0,
                            TextBoxFeatures_ReadText = 0x1,
                            TextBoxFeatures_WriteText = 0x2,
                            TextBoxFeatures_AugmentText = 0x4,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(TextBoxFeatures)
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextBoxSettings
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        enum TextBoxSettings : unsigned int
                        {
                            TextBoxSettings_None = 0,
                            TextBoxSettings_Private = 0x1,
                            TextBoxSettings_Multiline = 0x2,
                            TextBoxSettings_VerticalWriting = 0x4,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(TextBoxSettings)
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextChangeSource
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        enum TextChangeSource : int
                        {
                            TextChangeSource_External = 0,
                            TextChangeSource_HardwareKeyTyped = 1,
                            TextChangeSource_SoftwareKeyTyped = 2,
                            TextChangeSource_KeyboardImeInsertion = 3,
                            TextChangeSource_OtherImeInsertion = 4,
                            TextChangeSource_Reconversion = 5,
                            TextChangeSource_AutoCompletion = 6,
                            TextChangeSource_Mixed = 7,
                        };
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextConversionMode
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        enum TextConversionMode : int
                        {
                            TextConversionMode_Undefined = 0,
                            TextConversionMode_AlphanumericHalfWidth = 1,
                            TextConversionMode_AlphanumericFullWidth = 2,
                            TextConversionMode_NativeHalfWidth = 3,
                            TextConversionMode_NativeFullWidth = 4,
                            TextConversionMode_KatakanaHalfWidth = 5,
                            TextConversionMode_KatakanaFullWidth = 6,
                            TextConversionMode_NativeHalfWidthNativeSymbol = 7,
                            TextConversionMode_NativeFullWidthNativeSymbol = 8,
                            TextConversionMode_NoConversion = 9,
                            TextConversionMode_RequestConversion = 10,
                            TextConversionMode_NativeEudc = 11,
                        };
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextStyleAttributes
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        enum TextStyleAttributes : unsigned int
                        {
                            TextStyleAttributes_None = 0,
                            TextStyleAttributes_TextColor = 0x1,
                            TextStyleAttributes_BackgroundColor = 0x2,
                            TextStyleAttributes_UnderlineColor = 0x4,
                            TextStyleAttributes_UnderlineType = 0x8,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(TextStyleAttributes)
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextBoxId
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        struct TextBoxId
                        {
                            UINT32 Value;
                        };
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextInputServiceSubscription
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        struct TextInputServiceSubscription
                        {
                            ABI::Windows::UI::Input::Preview::Text::TextBoxFeatures requiredEnabledFeatures;
                            ABI::Windows::UI::Input::Preview::Text::TextBoxFeatures requiredDisabledFeatures;
                        };
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextStyle
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        struct TextStyle
                        {
                            ABI::Windows::UI::Input::Preview::Text::TextStyleAttributes mask;
                            ABI::Windows::UI::Color textColor;
                            ABI::Windows::UI::Color backgroundColor;
                            ABI::Windows::UI::Color underlineColor;
                            ABI::Windows::UI::Text::UnderlineType underlineType;
                        };
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IConversionModeChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.ConversionModeChangedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IConversionModeChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.IConversionModeChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("b49761f9-5b21-513c-b6c0-78f27d26b010")
                        IConversionModeChangedEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_NewConversionMode(
                                ABI::Windows::UI::Input::Preview::Text::TextConversionMode* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IConversionModeChangedEventArgs = __uuidof(IConversionModeChangedEventArgs);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IFocusEnteredEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.FocusEnteredEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IFocusEnteredEventArgs[] = L"Windows.UI.Input.Preview.Text.IFocusEnteredEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("ca4dc200-875f-501d-af14-413a0aa1ed5f")
                        IFocusEnteredEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_FocusedTextBoxInfo(
                                ABI::Windows::UI::Input::Preview::Text::ITextBoxInfo** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IFocusEnteredEventArgs = __uuidof(IFocusEnteredEventArgs);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IInputDelegationModeChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.InputDelegationModeChangedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IInputDelegationModeChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.IInputDelegationModeChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("4bb448b2-67ba-5215-8783-b444bd28eed3")
                        IInputDelegationModeChangedEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_DelegationOn(
                                boolean* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInputDelegationModeChangedEventArgs = __uuidof(IInputDelegationModeChangedEventArgs);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IKeyEventReceivedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.KeyEventReceivedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IKeyEventReceivedEventArgs[] = L"Windows.UI.Input.Preview.Text.IKeyEventReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("0c30f686-a058-5ecc-abd2-9cc861c1185b")
                        IKeyEventReceivedEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_VirtualKey(
                                ABI::Windows::System::VirtualKey* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_KeyStatus(
                                ABI::Windows::UI::Core::CorePhysicalKeyStatus* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Unicode(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Source(
                                ABI::Windows::UI::Input::Preview::Text::KeyEventDeviceType* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE IsKeyPressed(
                                ABI::Windows::System::VirtualKey vkey,
                                boolean* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE IsToggleKeyOn(
                                ABI::Windows::System::VirtualKey vkey,
                                boolean* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_EditSession(
                                ABI::Windows::UI::Input::Preview::Text::ITextEditSession** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Handled(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Handled(
                                boolean value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IKeyEventReceivedEventArgs = __uuidof(IKeyEventReceivedEventArgs);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IKeyboardInputProcessor
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.KeyboardInputProcessor
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IKeyboardInputProcessor[] = L"Windows.UI.Input.Preview.Text.IKeyboardInputProcessor";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("2afe79b6-5818-50e0-8fa8-81bc96428c46")
                        IKeyboardInputProcessor : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_InputProfile(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_HasFocusedTextBox(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_FocusedTextBoxId(
                                ABI::Windows::UI::Input::Preview::Text::TextBoxId* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_FocusedTextBoxInfo(
                                ABI::Windows::UI::Input::Preview::Text::ITextBoxInfo** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_FocusedTextBoxBounds(
                                __FIReference_1_Windows__CFoundation__CRect** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_SelectionBounds(
                                __FIReference_1_Windows__CFoundation__CRect** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ConversionMode(
                                ABI::Windows::UI::Input::Preview::Text::TextConversionMode* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_ConversionMode(
                                ABI::Windows::UI::Input::Preview::Text::TextConversionMode value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateEditSession(
                                ABI::Windows::UI::Input::Preview::Text::ITextEditSession** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_Activated(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_Activated(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_Deactivated(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_Deactivated(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_KeyEventReceived(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_KeyEventReceived(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_FocusEntered(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_FocusEntered(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_FocusRemoved(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_FocusRemoved(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_ConversionModeChanged(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_ConversionModeChanged(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_TextBoxInfoChanged(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_TextBoxInfoChanged(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_TextBoxContentChanged(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_TextBoxContentChanged(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_CompositionTerminated(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_CompositionTerminated(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_ReconversionRequested(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_ReconversionRequested(
                                EventRegistrationToken token
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IKeyboardInputProcessor = __uuidof(IKeyboardInputProcessor);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IReconversionRequestedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IReconversionRequestedEventArgs[] = L"Windows.UI.Input.Preview.Text.IReconversionRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("73852244-d202-55fe-9edf-beb7ec19f937")
                        IReconversionRequestedEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Range(
                                ABI::Windows::UI::Text::Core::CoreTextRange* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IReconversionRequestedEventArgs = __uuidof(IReconversionRequestedEventArgs);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextBoxContentChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextBoxContentChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.ITextBoxContentChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("2cb70a41-5aed-58c5-b4c1-8ee4e1492f9e")
                        ITextBoxContentChangedEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_TextBoxId(
                                ABI::Windows::UI::Input::Preview::Text::TextBoxId* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Source(
                                ABI::Windows::UI::Input::Preview::Text::TextChangeSource* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_SelectionBounds(
                                ABI::Windows::Foundation::Rect* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE IsContentAttributeChanged(
                                ABI::Windows::UI::Input::Preview::Text::TextBoxContentAttribute value,
                                boolean* result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextBoxContentChangedEventArgs = __uuidof(ITextBoxContentChangedEventArgs);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextBoxInfo
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextBoxInfo
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextBoxInfo[] = L"Windows.UI.Input.Preview.Text.ITextBoxInfo";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("b122443d-e8f7-5f8b-813d-aaa0941d5fa0")
                        ITextBoxInfo : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Id(
                                ABI::Windows::UI::Input::Preview::Text::TextBoxId* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_InputScope(
                                ABI::Windows::UI::Text::Core::CoreTextInputScope* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AppName(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Url(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Settings(
                                ABI::Windows::UI::Input::Preview::Text::TextBoxSettings* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DisabledFeatures(
                                ABI::Windows::UI::Input::Preview::Text::TextBoxFeatures* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextBoxInfo = __uuidof(ITextBoxInfo);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextBoxInfoChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextBoxInfoChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.ITextBoxInfoChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("ac1275af-648c-5bac-b29f-d1ea17e9e6d6")
                        ITextBoxInfoChangedEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_TextBoxInfo(
                                ABI::Windows::UI::Input::Preview::Text::ITextBoxInfo** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextBoxInfoChangedEventArgs = __uuidof(ITextBoxInfoChangedEventArgs);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextComposition
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextComposition
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextComposition[] = L"Windows.UI.Input.Preview.Text.ITextComposition";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("5cea9aea-524d-50a4-b08a-c83d8d25ec6e")
                        ITextComposition : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Text(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_FirstSegment(
                                ABI::Windows::UI::Input::Preview::Text::ITextCompositionSegment** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_SelectedSegment(
                                ABI::Windows::UI::Input::Preview::Text::ITextCompositionSegment** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_CaretPosition(
                                UINT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CaretPosition(
                                UINT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InsertText(
                                HSTRING text,
                                ABI::Windows::UI::Input::Preview::Text::ITextCompositionSegment** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CompleteUnconverted(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CompleteFirstSegment(void) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextComposition = __uuidof(ITextComposition);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextCompositionSegment
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextCompositionSegment
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextCompositionSegment[] = L"Windows.UI.Input.Preview.Text.ITextCompositionSegment";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("0543f6c6-eb98-56d6-8808-2eca6d02f6a5")
                        ITextCompositionSegment : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Text(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Text(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ConvertedText(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_ConvertedText(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_UnconvertedText(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_UnconvertedText(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Range(
                                ABI::Windows::UI::Text::Core::CoreTextRange* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ConversionState(
                                ABI::Windows::UI::Text::Core::CoreTextFormatUpdatingReason* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_ConversionState(
                                ABI::Windows::UI::Text::Core::CoreTextFormatUpdatingReason value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Next(
                                ABI::Windows::UI::Input::Preview::Text::ITextCompositionSegment** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Previous(
                                ABI::Windows::UI::Input::Preview::Text::ITextCompositionSegment** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetTextStyle(
                                ABI::Windows::UI::Input::Preview::Text::TextStyle* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetTextStyle(
                                ABI::Windows::UI::Input::Preview::Text::TextStyle value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextCompositionSegment = __uuidof(ITextCompositionSegment);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextEditSession
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextEditSession
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextEditSession[] = L"Windows.UI.Input.Preview.Text.ITextEditSession";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("0bcad18a-d31b-5787-aff9-995ee743aea8")
                        ITextEditSession : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_TextBoxId(
                                ABI::Windows::UI::Input::Preview::Text::TextBoxId* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_TextLength(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PopulatedRange(
                                ABI::Windows::UI::Text::Core::CoreTextRange* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE PopulateAsync(
                                ABI::Windows::UI::Text::Core::CoreTextRange range,
                                ABI::Windows::Foundation::IAsyncAction** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetText(
                                ABI::Windows::UI::Text::Core::CoreTextRange range,
                                HSTRING* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetSelectedRange(
                                ABI::Windows::UI::Text::Core::CoreTextRange* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetSelectedRange(
                                ABI::Windows::UI::Text::Core::CoreTextRange value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ReplaceText(
                                ABI::Windows::UI::Text::Core::CoreTextRange replaceRange,
                                HSTRING text,
                                ABI::Windows::UI::Text::Core::CoreTextRange* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Composition(
                                ABI::Windows::UI::Input::Preview::Text::ITextComposition** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE StartComposition(
                                ABI::Windows::UI::Input::Preview::Text::ITextComposition** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE StartReconversion(
                                ABI::Windows::UI::Text::Core::CoreTextRange range,
                                ABI::Windows::UI::Input::Preview::Text::ITextComposition** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SubmitPayload(
                                boolean* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SubmitPayloadAsync(
                                __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextEditSession = __uuidof(ITextEditSession);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextInputProvider
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextInputProvider
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextInputProvider[] = L"Windows.UI.Input.Preview.Text.ITextInputProvider";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("b0885fb7-e9f8-5849-b0ef-f8155ecf60d1")
                        ITextInputProvider : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE GetSubscription(
                                ABI::Windows::UI::Input::Preview::Text::TextInputServiceSubscription* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetSubscription(
                                ABI::Windows::UI::Input::Preview::Text::TextInputServiceSubscription subscription
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_HasFocusedTextBox(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_FocusedTextBoxId(
                                ABI::Windows::UI::Input::Preview::Text::TextBoxId* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_FocusedTextBoxInfo(
                                ABI::Windows::UI::Input::Preview::Text::ITextBoxInfo** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_FocusedTextBoxBounds(
                                __FIReference_1_Windows__CFoundation__CRect** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_SelectionBounds(
                                __FIReference_1_Windows__CFoundation__CRect** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateEditSession(
                                ABI::Windows::UI::Input::Preview::Text::ITextEditSession** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE TryStartDelegation(
                                boolean* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE StopDelegation(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_FocusEntered(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_FocusEntered(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_FocusRemoved(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_FocusRemoved(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_TextBoxInfoChanged(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_TextBoxInfoChanged(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_TextBoxContentChanged(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_TextBoxContentChanged(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_CompositionTerminated(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_CompositionTerminated(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_ReconversionRequested(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_ReconversionRequested(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_InputDelegationModeChanged(
                                __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_InputDelegationModeChanged(
                                EventRegistrationToken token
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextInputProvider = __uuidof(ITextInputProvider);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextInputService
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextInputService
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextInputService[] = L"Windows.UI.Input.Preview.Text.ITextInputService";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("8e23f89c-ab1f-551a-8751-7d4f29e34d88")
                        ITextInputService : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateKeyboardInputProcessor(
                                HSTRING inputProfile,
                                ABI::Windows::UI::Input::Preview::Text::IKeyboardInputProcessor** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateTextInputProvider(
                                HSTRING inputProfile,
                                ABI::Windows::UI::Input::Preview::Text::ITextInputProvider** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextInputService = __uuidof(ITextInputService);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextInputServiceStatics
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextInputService
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextInputServiceStatics[] = L"Windows.UI.Input.Preview.Text.ITextInputServiceStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Text {
                        MIDL_INTERFACE("91b68f5e-02ed-4e09-ae89-dfd735cf10bc")
                        ITextInputServiceStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE GetForCurrentThread(
                                ABI::Windows::UI::Input::Preview::Text::ITextInputService** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ITextInputServiceStatics = __uuidof(ITextInputServiceStatics);
                    } /* Text */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.ConversionModeChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IConversionModeChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_ConversionModeChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_ConversionModeChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_ConversionModeChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.ConversionModeChangedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.FocusEnteredEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IFocusEnteredEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_FocusEnteredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_FocusEnteredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_FocusEnteredEventArgs[] = L"Windows.UI.Input.Preview.Text.FocusEnteredEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.InputDelegationModeChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IInputDelegationModeChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_InputDelegationModeChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_InputDelegationModeChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_InputDelegationModeChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.InputDelegationModeChangedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.KeyEventReceivedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IKeyEventReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_KeyEventReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_KeyEventReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_KeyEventReceivedEventArgs[] = L"Windows.UI.Input.Preview.Text.KeyEventReceivedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.KeyboardInputProcessor
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IKeyboardInputProcessor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_KeyboardInputProcessor_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_KeyboardInputProcessor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_KeyboardInputProcessor[] = L"Windows.UI.Input.Preview.Text.KeyboardInputProcessor";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IReconversionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_ReconversionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_ReconversionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_ReconversionRequestedEventArgs[] = L"Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextBoxContentChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxContentChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxContentChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextBoxContentChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextBoxInfo
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextBoxInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextBoxInfo[] = L"Windows.UI.Input.Preview.Text.TextBoxInfo";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextBoxInfoChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxInfoChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxInfoChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextBoxInfoChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextComposition
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextComposition ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextComposition_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextComposition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextComposition[] = L"Windows.UI.Input.Preview.Text.TextComposition";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextCompositionSegment
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextCompositionSegment ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextCompositionSegment_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextCompositionSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextCompositionSegment[] = L"Windows.UI.Input.Preview.Text.TextCompositionSegment";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextEditSession
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextEditSession ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextEditSession_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextEditSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextEditSession[] = L"Windows.UI.Input.Preview.Text.TextEditSession";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextInputProvider
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextInputProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextInputProvider_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextInputProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextInputProvider[] = L"Windows.UI.Input.Preview.Text.TextInputProvider";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextInputService
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Preview.Text.ITextInputServiceStatics interface starting with version 1.0 of the Windows.UI.Input.Preview.Text.PreviewTextContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextInputService ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextInputService_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextInputService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextInputService[] = L"Windows.UI.Input.Preview.Text.TextInputService";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CPayloadResult __x_ABI_CWindows_CUI_CInput_CPreview_CText_CPayloadResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult;

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult;

typedef struct __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CPayloadResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResultVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* This,
        __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CRect_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CRect_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CRect __FIReference_1_Windows__CFoundation__CRect;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CRect;

typedef struct __FIReference_1_Windows__CFoundation__CRectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CRect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CRect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CRect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CRect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CRect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CRect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CRect* This,
        struct __x_ABI_CWindows_CFoundation_CRect* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CRectVtbl;

interface __FIReference_1_Windows__CFoundation__CRect
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CRectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CRect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CRect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CRect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CRect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CRect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CRect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CRect_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CRect_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* sender,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CVirtualKey __x_ABI_CWindows_CSystem_CVirtualKey;

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef struct __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus;

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingReason __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingReason;

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope;

typedef struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange;

typedef enum __x_ABI_CWindows_CUI_CText_CUnderlineType __x_ABI_CWindows_CUI_CText_CUnderlineType;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CKeyEventDeviceType __x_ABI_CWindows_CUI_CInput_CPreview_CText_CKeyEventDeviceType;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxContentAttribute __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxContentAttribute;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxFeatures __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxFeatures;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxSettings __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxSettings;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextChangeSource __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextChangeSource;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextConversionMode __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextConversionMode;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyleAttributes __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyleAttributes;

typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxId __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxId;

typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextInputServiceSubscription __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextInputServiceSubscription;

typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyle __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyle;

/*
 *
 * Struct Windows.UI.Input.Preview.Text.KeyEventDeviceType
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CKeyEventDeviceType
{
    KeyEventDeviceType_Undefined = 0,
    KeyEventDeviceType_HardwareKeyboard = 1,
    KeyEventDeviceType_SoftwareKeyboard = 2,
    KeyEventDeviceType_Gamepad = 3,
    KeyEventDeviceType_Injection = 4,
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.PayloadResult
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CPayloadResult
{
    PayloadResult_InEditing = 0,
    PayloadResult_Pending = 1,
    PayloadResult_Completed = 2,
    PayloadResult_Overridden = 3,
    PayloadResult_Outrun = 4,
    PayloadResult_Rejected = 5,
    PayloadResult_Canceled = 6,
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextBoxContentAttribute
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxContentAttribute
{
    TextBoxContentAttribute_None = 0,
    TextBoxContentAttribute_Selection = 1,
    TextBoxContentAttribute_Text = 2,
    TextBoxContentAttribute_Property = 3,
    TextBoxContentAttribute_Layout = 4,
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextBoxFeatures
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxFeatures
{
    TextBoxFeatures_None = 0,
    TextBoxFeatures_ReadText = 0x1,
    TextBoxFeatures_WriteText = 0x2,
    TextBoxFeatures_AugmentText = 0x4,
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextBoxSettings
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxSettings
{
    TextBoxSettings_None = 0,
    TextBoxSettings_Private = 0x1,
    TextBoxSettings_Multiline = 0x2,
    TextBoxSettings_VerticalWriting = 0x4,
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextChangeSource
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextChangeSource
{
    TextChangeSource_External = 0,
    TextChangeSource_HardwareKeyTyped = 1,
    TextChangeSource_SoftwareKeyTyped = 2,
    TextChangeSource_KeyboardImeInsertion = 3,
    TextChangeSource_OtherImeInsertion = 4,
    TextChangeSource_Reconversion = 5,
    TextChangeSource_AutoCompletion = 6,
    TextChangeSource_Mixed = 7,
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextConversionMode
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextConversionMode
{
    TextConversionMode_Undefined = 0,
    TextConversionMode_AlphanumericHalfWidth = 1,
    TextConversionMode_AlphanumericFullWidth = 2,
    TextConversionMode_NativeHalfWidth = 3,
    TextConversionMode_NativeFullWidth = 4,
    TextConversionMode_KatakanaHalfWidth = 5,
    TextConversionMode_KatakanaFullWidth = 6,
    TextConversionMode_NativeHalfWidthNativeSymbol = 7,
    TextConversionMode_NativeFullWidthNativeSymbol = 8,
    TextConversionMode_NoConversion = 9,
    TextConversionMode_RequestConversion = 10,
    TextConversionMode_NativeEudc = 11,
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextStyleAttributes
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyleAttributes
{
    TextStyleAttributes_None = 0,
    TextStyleAttributes_TextColor = 0x1,
    TextStyleAttributes_BackgroundColor = 0x2,
    TextStyleAttributes_UnderlineColor = 0x4,
    TextStyleAttributes_UnderlineType = 0x8,
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextBoxId
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxId
{
    UINT32 Value;
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextInputServiceSubscription
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextInputServiceSubscription
{
    enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxFeatures requiredEnabledFeatures;
    enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxFeatures requiredDisabledFeatures;
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Preview.Text.TextStyle
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyle
{
    enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyleAttributes mask;
    struct __x_ABI_CWindows_CUI_CColor textColor;
    struct __x_ABI_CWindows_CUI_CColor backgroundColor;
    struct __x_ABI_CWindows_CUI_CColor underlineColor;
    enum __x_ABI_CWindows_CUI_CText_CUnderlineType underlineType;
};
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IConversionModeChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.ConversionModeChangedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IConversionModeChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.IConversionModeChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NewConversionMode)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextConversionMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_get_NewConversionMode(This, value) \
    ((This)->lpVtbl->get_NewConversionMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIConversionModeChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IFocusEnteredEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.FocusEnteredEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IFocusEnteredEventArgs[] = L"Windows.UI.Input.Preview.Text.IFocusEnteredEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FocusedTextBoxInfo)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_get_FocusedTextBoxInfo(This, value) \
    ((This)->lpVtbl->get_FocusedTextBoxInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIFocusEnteredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IInputDelegationModeChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.InputDelegationModeChangedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IInputDelegationModeChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.IInputDelegationModeChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DelegationOn)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_get_DelegationOn(This, value) \
    ((This)->lpVtbl->get_DelegationOn(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIInputDelegationModeChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IKeyEventReceivedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.KeyEventReceivedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IKeyEventReceivedEventArgs[] = L"Windows.UI.Input.Preview.Text.IKeyEventReceivedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VirtualKey)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyStatus)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Unicode)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CKeyEventDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* IsKeyPressed)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey vkey,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsToggleKeyOn)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey vkey,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_EditSession)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession** value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_get_VirtualKey(This, value) \
    ((This)->lpVtbl->get_VirtualKey(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_get_KeyStatus(This, value) \
    ((This)->lpVtbl->get_KeyStatus(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_get_Unicode(This, value) \
    ((This)->lpVtbl->get_Unicode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_IsKeyPressed(This, vkey, result) \
    ((This)->lpVtbl->IsKeyPressed(This, vkey, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_IsToggleKeyOn(This, vkey, result) \
    ((This)->lpVtbl->IsToggleKeyOn(This, vkey, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_get_EditSession(This, value) \
    ((This)->lpVtbl->get_EditSession(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyEventReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IKeyboardInputProcessor
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.KeyboardInputProcessor
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IKeyboardInputProcessor[] = L"Windows.UI.Input.Preview.Text.IKeyboardInputProcessor";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InputProfile)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasFocusedTextBox)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_FocusedTextBoxId)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxId* value);
    HRESULT (STDMETHODCALLTYPE* get_FocusedTextBoxInfo)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_FocusedTextBoxBounds)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FIReference_1_Windows__CFoundation__CRect** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionBounds)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FIReference_1_Windows__CFoundation__CRect** value);
    HRESULT (STDMETHODCALLTYPE* get_ConversionMode)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextConversionMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ConversionMode)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextConversionMode value);
    HRESULT (STDMETHODCALLTYPE* CreateEditSession)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession** result);
    HRESULT (STDMETHODCALLTYPE* add_Activated)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Activated)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Deactivated)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Deactivated)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_KeyEventReceived)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CKeyEventReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_KeyEventReceived)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_FocusEntered)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FocusEntered)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_FocusRemoved)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FocusRemoved)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ConversionModeChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CConversionModeChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ConversionModeChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TextBoxInfoChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TextBoxInfoChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TextBoxContentChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TextBoxContentChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CompositionTerminated)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CompositionTerminated)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ReconversionRequested)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CKeyboardInputProcessor_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ReconversionRequested)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessorVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_get_InputProfile(This, value) \
    ((This)->lpVtbl->get_InputProfile(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_get_HasFocusedTextBox(This, value) \
    ((This)->lpVtbl->get_HasFocusedTextBox(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_get_FocusedTextBoxId(This, value) \
    ((This)->lpVtbl->get_FocusedTextBoxId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_get_FocusedTextBoxInfo(This, value) \
    ((This)->lpVtbl->get_FocusedTextBoxInfo(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_get_FocusedTextBoxBounds(This, value) \
    ((This)->lpVtbl->get_FocusedTextBoxBounds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_get_SelectionBounds(This, value) \
    ((This)->lpVtbl->get_SelectionBounds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_get_ConversionMode(This, value) \
    ((This)->lpVtbl->get_ConversionMode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_put_ConversionMode(This, value) \
    ((This)->lpVtbl->put_ConversionMode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_CreateEditSession(This, result) \
    ((This)->lpVtbl->CreateEditSession(This, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_Activated(This, handler, token) \
    ((This)->lpVtbl->add_Activated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_Activated(This, token) \
    ((This)->lpVtbl->remove_Activated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_Deactivated(This, handler, token) \
    ((This)->lpVtbl->add_Deactivated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_Deactivated(This, token) \
    ((This)->lpVtbl->remove_Deactivated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_KeyEventReceived(This, handler, token) \
    ((This)->lpVtbl->add_KeyEventReceived(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_KeyEventReceived(This, token) \
    ((This)->lpVtbl->remove_KeyEventReceived(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_FocusEntered(This, handler, token) \
    ((This)->lpVtbl->add_FocusEntered(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_FocusEntered(This, token) \
    ((This)->lpVtbl->remove_FocusEntered(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_FocusRemoved(This, handler, token) \
    ((This)->lpVtbl->add_FocusRemoved(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_FocusRemoved(This, token) \
    ((This)->lpVtbl->remove_FocusRemoved(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_ConversionModeChanged(This, handler, token) \
    ((This)->lpVtbl->add_ConversionModeChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_ConversionModeChanged(This, token) \
    ((This)->lpVtbl->remove_ConversionModeChanged(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_TextBoxInfoChanged(This, handler, token) \
    ((This)->lpVtbl->add_TextBoxInfoChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_TextBoxInfoChanged(This, token) \
    ((This)->lpVtbl->remove_TextBoxInfoChanged(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_TextBoxContentChanged(This, handler, token) \
    ((This)->lpVtbl->add_TextBoxContentChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_TextBoxContentChanged(This, token) \
    ((This)->lpVtbl->remove_TextBoxContentChanged(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_CompositionTerminated(This, handler, token) \
    ((This)->lpVtbl->add_CompositionTerminated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_CompositionTerminated(This, token) \
    ((This)->lpVtbl->remove_CompositionTerminated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_add_ReconversionRequested(This, handler, token) \
    ((This)->lpVtbl->add_ReconversionRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_remove_ReconversionRequested(This, token) \
    ((This)->lpVtbl->remove_ReconversionRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.IReconversionRequestedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_IReconversionRequestedEventArgs[] = L"Windows.UI.Input.Preview.Text.IReconversionRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Range)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_get_Range(This, value) \
    ((This)->lpVtbl->get_Range(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CIReconversionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextBoxContentChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextBoxContentChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.ITextBoxContentChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextBoxId)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxId* value);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextChangeSource* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionBounds)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* IsContentAttributeChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxContentAttribute value,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_get_TextBoxId(This, value) \
    ((This)->lpVtbl->get_TextBoxId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_get_SelectionBounds(This, value) \
    ((This)->lpVtbl->get_SelectionBounds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_IsContentAttributeChanged(This, value, result) \
    ((This)->lpVtbl->IsContentAttributeChanged(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxContentChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextBoxInfo
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextBoxInfo
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextBoxInfo[] = L"Windows.UI.Input.Preview.Text.ITextBoxInfo";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxId* value);
    HRESULT (STDMETHODCALLTYPE* get_InputScope)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope* value);
    HRESULT (STDMETHODCALLTYPE* get_AppName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Url)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Settings)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxSettings* value);
    HRESULT (STDMETHODCALLTYPE* get_DisabledFeatures)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxFeatures* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_get_InputScope(This, value) \
    ((This)->lpVtbl->get_InputScope(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_get_AppName(This, value) \
    ((This)->lpVtbl->get_AppName(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_get_Url(This, value) \
    ((This)->lpVtbl->get_Url(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_get_Settings(This, value) \
    ((This)->lpVtbl->get_Settings(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_get_DisabledFeatures(This, value) \
    ((This)->lpVtbl->get_DisabledFeatures(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextBoxInfoChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextBoxInfoChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.ITextBoxInfoChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextBoxInfo)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_get_TextBoxInfo(This, value) \
    ((This)->lpVtbl->get_TextBoxInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfoChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextComposition
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextComposition
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextComposition[] = L"Windows.UI.Input.Preview.Text.ITextComposition";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FirstSegment)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedSegment)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment** value);
    HRESULT (STDMETHODCALLTYPE* get_CaretPosition)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_CaretPosition)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* InsertText)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This,
        HSTRING text,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment** result);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This);
    HRESULT (STDMETHODCALLTYPE* CompleteUnconverted)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This);
    HRESULT (STDMETHODCALLTYPE* CompleteFirstSegment)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_get_FirstSegment(This, value) \
    ((This)->lpVtbl->get_FirstSegment(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_get_SelectedSegment(This, value) \
    ((This)->lpVtbl->get_SelectedSegment(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_get_CaretPosition(This, value) \
    ((This)->lpVtbl->get_CaretPosition(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_put_CaretPosition(This, value) \
    ((This)->lpVtbl->put_CaretPosition(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_InsertText(This, text, result) \
    ((This)->lpVtbl->InsertText(This, text, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_CompleteUnconverted(This) \
    ((This)->lpVtbl->CompleteUnconverted(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_CompleteFirstSegment(This) \
    ((This)->lpVtbl->CompleteFirstSegment(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextCompositionSegment
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextCompositionSegment
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextCompositionSegment[] = L"Windows.UI.Input.Preview.Text.ITextCompositionSegment";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ConvertedText)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ConvertedText)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_UnconvertedText)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_UnconvertedText)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Range)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* get_ConversionState)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingReason* value);
    HRESULT (STDMETHODCALLTYPE* put_ConversionState)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingReason value);
    HRESULT (STDMETHODCALLTYPE* get_Next)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment** value);
    HRESULT (STDMETHODCALLTYPE* get_Previous)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment** value);
    HRESULT (STDMETHODCALLTYPE* GetTextStyle)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyle* result);
    HRESULT (STDMETHODCALLTYPE* SetTextStyle)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextStyle value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegmentVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_get_ConvertedText(This, value) \
    ((This)->lpVtbl->get_ConvertedText(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_put_ConvertedText(This, value) \
    ((This)->lpVtbl->put_ConvertedText(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_get_UnconvertedText(This, value) \
    ((This)->lpVtbl->get_UnconvertedText(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_put_UnconvertedText(This, value) \
    ((This)->lpVtbl->put_UnconvertedText(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_get_Range(This, value) \
    ((This)->lpVtbl->get_Range(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_get_ConversionState(This, value) \
    ((This)->lpVtbl->get_ConversionState(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_put_ConversionState(This, value) \
    ((This)->lpVtbl->put_ConversionState(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_get_Next(This, value) \
    ((This)->lpVtbl->get_Next(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_get_Previous(This, value) \
    ((This)->lpVtbl->get_Previous(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_GetTextStyle(This, result) \
    ((This)->lpVtbl->GetTextStyle(This, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_SetTextStyle(This, value) \
    ((This)->lpVtbl->SetTextStyle(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextCompositionSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextEditSession
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextEditSession
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextEditSession[] = L"Windows.UI.Input.Preview.Text.ITextEditSession";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextBoxId)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxId* value);
    HRESULT (STDMETHODCALLTYPE* get_TextLength)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PopulatedRange)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* PopulateAsync)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange range,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* GetText)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange range,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetSelectedRange)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* result);
    HRESULT (STDMETHODCALLTYPE* SetSelectedRange)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange value);
    HRESULT (STDMETHODCALLTYPE* ReplaceText)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange replaceRange,
        HSTRING text,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* result);
    HRESULT (STDMETHODCALLTYPE* get_Composition)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition** value);
    HRESULT (STDMETHODCALLTYPE* StartComposition)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition** result);
    HRESULT (STDMETHODCALLTYPE* StartReconversion)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange range,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextComposition** result);
    HRESULT (STDMETHODCALLTYPE* SubmitPayload)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SubmitPayloadAsync)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession* This,
        __FIAsyncOperation_1_Windows__CUI__CInput__CPreview__CText__CPayloadResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSessionVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_get_TextBoxId(This, value) \
    ((This)->lpVtbl->get_TextBoxId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_get_TextLength(This, value) \
    ((This)->lpVtbl->get_TextLength(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_get_PopulatedRange(This, value) \
    ((This)->lpVtbl->get_PopulatedRange(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_PopulateAsync(This, range, operation) \
    ((This)->lpVtbl->PopulateAsync(This, range, operation))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_GetText(This, range, result) \
    ((This)->lpVtbl->GetText(This, range, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_GetSelectedRange(This, result) \
    ((This)->lpVtbl->GetSelectedRange(This, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_SetSelectedRange(This, value) \
    ((This)->lpVtbl->SetSelectedRange(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_ReplaceText(This, replaceRange, text, result) \
    ((This)->lpVtbl->ReplaceText(This, replaceRange, text, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_get_Composition(This, value) \
    ((This)->lpVtbl->get_Composition(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_StartComposition(This, result) \
    ((This)->lpVtbl->StartComposition(This, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_StartReconversion(This, range, result) \
    ((This)->lpVtbl->StartReconversion(This, range, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_SubmitPayload(This, result) \
    ((This)->lpVtbl->SubmitPayload(This, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_SubmitPayloadAsync(This, operation) \
    ((This)->lpVtbl->SubmitPayloadAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextInputProvider
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextInputProvider
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextInputProvider[] = L"Windows.UI.Input.Preview.Text.ITextInputProvider";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetSubscription)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextInputServiceSubscription* result);
    HRESULT (STDMETHODCALLTYPE* SetSubscription)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextInputServiceSubscription subscription);
    HRESULT (STDMETHODCALLTYPE* get_HasFocusedTextBox)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_FocusedTextBoxId)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CTextBoxId* value);
    HRESULT (STDMETHODCALLTYPE* get_FocusedTextBoxInfo)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextBoxInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_FocusedTextBoxBounds)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FIReference_1_Windows__CFoundation__CRect** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionBounds)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FIReference_1_Windows__CFoundation__CRect** value);
    HRESULT (STDMETHODCALLTYPE* CreateEditSession)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextEditSession** result);
    HRESULT (STDMETHODCALLTYPE* TryStartDelegation)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* StopDelegation)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This);
    HRESULT (STDMETHODCALLTYPE* add_FocusEntered)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CFocusEnteredEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FocusEntered)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_FocusRemoved)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FocusRemoved)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TextBoxInfoChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxInfoChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TextBoxInfoChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TextBoxContentChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CTextBoxContentChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TextBoxContentChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CompositionTerminated)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CompositionTerminated)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ReconversionRequested)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CReconversionRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ReconversionRequested)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_InputDelegationModeChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CPreview__CText__CTextInputProvider_Windows__CUI__CInput__CPreview__CText__CInputDelegationModeChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_InputDelegationModeChanged)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProviderVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_GetSubscription(This, result) \
    ((This)->lpVtbl->GetSubscription(This, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_SetSubscription(This, subscription) \
    ((This)->lpVtbl->SetSubscription(This, subscription))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_get_HasFocusedTextBox(This, value) \
    ((This)->lpVtbl->get_HasFocusedTextBox(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_get_FocusedTextBoxId(This, value) \
    ((This)->lpVtbl->get_FocusedTextBoxId(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_get_FocusedTextBoxInfo(This, value) \
    ((This)->lpVtbl->get_FocusedTextBoxInfo(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_get_FocusedTextBoxBounds(This, value) \
    ((This)->lpVtbl->get_FocusedTextBoxBounds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_get_SelectionBounds(This, value) \
    ((This)->lpVtbl->get_SelectionBounds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_CreateEditSession(This, result) \
    ((This)->lpVtbl->CreateEditSession(This, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_TryStartDelegation(This, result) \
    ((This)->lpVtbl->TryStartDelegation(This, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_StopDelegation(This) \
    ((This)->lpVtbl->StopDelegation(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_add_FocusEntered(This, handler, token) \
    ((This)->lpVtbl->add_FocusEntered(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_remove_FocusEntered(This, token) \
    ((This)->lpVtbl->remove_FocusEntered(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_add_FocusRemoved(This, handler, token) \
    ((This)->lpVtbl->add_FocusRemoved(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_remove_FocusRemoved(This, token) \
    ((This)->lpVtbl->remove_FocusRemoved(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_add_TextBoxInfoChanged(This, handler, token) \
    ((This)->lpVtbl->add_TextBoxInfoChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_remove_TextBoxInfoChanged(This, token) \
    ((This)->lpVtbl->remove_TextBoxInfoChanged(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_add_TextBoxContentChanged(This, handler, token) \
    ((This)->lpVtbl->add_TextBoxContentChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_remove_TextBoxContentChanged(This, token) \
    ((This)->lpVtbl->remove_TextBoxContentChanged(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_add_CompositionTerminated(This, handler, token) \
    ((This)->lpVtbl->add_CompositionTerminated(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_remove_CompositionTerminated(This, token) \
    ((This)->lpVtbl->remove_CompositionTerminated(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_add_ReconversionRequested(This, handler, token) \
    ((This)->lpVtbl->add_ReconversionRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_remove_ReconversionRequested(This, token) \
    ((This)->lpVtbl->remove_ReconversionRequested(This, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_add_InputDelegationModeChanged(This, handler, token) \
    ((This)->lpVtbl->add_InputDelegationModeChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_remove_InputDelegationModeChanged(This, token) \
    ((This)->lpVtbl->remove_InputDelegationModeChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextInputService
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextInputService
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextInputService[] = L"Windows.UI.Input.Preview.Text.ITextInputService";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateKeyboardInputProcessor)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService* This,
        HSTRING inputProfile,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CIKeyboardInputProcessor** result);
    HRESULT (STDMETHODCALLTYPE* CreateTextInputProvider)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService* This,
        HSTRING inputProfile,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputProvider** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_CreateKeyboardInputProcessor(This, inputProfile, result) \
    ((This)->lpVtbl->CreateKeyboardInputProcessor(This, inputProfile, result))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_CreateTextInputProvider(This, inputProfile, result) \
    ((This)->lpVtbl->CreateTextInputProvider(This, inputProfile, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Preview.Text.ITextInputServiceStatics
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Text.TextInputService
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Text_ITextInputServiceStatics[] = L"Windows.UI.Input.Preview.Text.ITextInputServiceStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentThread)(__x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputService** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_GetForCurrentThread(This, result) \
    ((This)->lpVtbl->GetForCurrentThread(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CText_CITextInputServiceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.ConversionModeChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IConversionModeChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_ConversionModeChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_ConversionModeChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_ConversionModeChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.ConversionModeChangedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.FocusEnteredEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IFocusEnteredEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_FocusEnteredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_FocusEnteredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_FocusEnteredEventArgs[] = L"Windows.UI.Input.Preview.Text.FocusEnteredEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.InputDelegationModeChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IInputDelegationModeChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_InputDelegationModeChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_InputDelegationModeChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_InputDelegationModeChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.InputDelegationModeChangedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.KeyEventReceivedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IKeyEventReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_KeyEventReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_KeyEventReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_KeyEventReceivedEventArgs[] = L"Windows.UI.Input.Preview.Text.KeyEventReceivedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.KeyboardInputProcessor
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IKeyboardInputProcessor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_KeyboardInputProcessor_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_KeyboardInputProcessor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_KeyboardInputProcessor[] = L"Windows.UI.Input.Preview.Text.KeyboardInputProcessor";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.IReconversionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_ReconversionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_ReconversionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_ReconversionRequestedEventArgs[] = L"Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextBoxContentChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxContentChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxContentChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextBoxContentChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextBoxInfo
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextBoxInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextBoxInfo[] = L"Windows.UI.Input.Preview.Text.TextBoxInfo";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextBoxInfoChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxInfoChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextBoxInfoChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextBoxInfoChangedEventArgs[] = L"Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextComposition
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextComposition ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextComposition_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextComposition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextComposition[] = L"Windows.UI.Input.Preview.Text.TextComposition";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextCompositionSegment
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextCompositionSegment ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextCompositionSegment_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextCompositionSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextCompositionSegment[] = L"Windows.UI.Input.Preview.Text.TextCompositionSegment";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextEditSession
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextEditSession ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextEditSession_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextEditSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextEditSession[] = L"Windows.UI.Input.Preview.Text.TextEditSession";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextInputProvider
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextInputProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextInputProvider_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextInputProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextInputProvider[] = L"Windows.UI.Input.Preview.Text.TextInputProvider";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Preview.Text.TextInputService
 *
 * Introduced to Windows.UI.Input.Preview.Text.PreviewTextContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Preview.Text.ITextInputServiceStatics interface starting with version 1.0 of the Windows.UI.Input.Preview.Text.PreviewTextContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Text.ITextInputService ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextInputService_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Text_TextInputService_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Text_TextInputService[] = L"Windows.UI.Input.Preview.Text.TextInputService";
#endif
#endif // WINDOWS_UI_INPUT_PREVIEW_TEXT_PREVIEWTEXTCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Einput2Epreview2Etext_p_h__

#endif // __windows2Eui2Einput2Epreview2Etext_h__
