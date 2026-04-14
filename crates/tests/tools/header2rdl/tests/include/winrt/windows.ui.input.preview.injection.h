
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
#ifndef __windows2Eui2Einput2Epreview2Einjection_h__
#define __windows2Eui2Einput2Epreview2Einjection_h__
#ifndef __windows2Eui2Einput2Epreview2Einjection_p_h__
#define __windows2Eui2Einput2Epreview2Einjection_p_h__


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

#if !defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)
#define WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Gaming.Input.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInjectedInputGamepadInfo;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo ABI::Windows::UI::Input::Preview::Injection::IInjectedInputGamepadInfo

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInjectedInputGamepadInfoFactory;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory ABI::Windows::UI::Input::Preview::Injection::IInjectedInputGamepadInfoFactory

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInjectedInputKeyboardInfo;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo ABI::Windows::UI::Input::Preview::Injection::IInjectedInputKeyboardInfo

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInjectedInputMouseInfo;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo ABI::Windows::UI::Input::Preview::Injection::IInjectedInputMouseInfo

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInjectedInputPenInfo;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo ABI::Windows::UI::Input::Preview::Injection::IInjectedInputPenInfo

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInjectedInputTouchInfo;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo ABI::Windows::UI::Input::Preview::Injection::IInjectedInputTouchInfo

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInputInjector;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector ABI::Windows::UI::Input::Preview::Injection::IInputInjector

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInputInjector2;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2 ABI::Windows::UI::Input::Preview::Injection::IInputInjector2

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInputInjectorStatics;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics ABI::Windows::UI::Input::Preview::Injection::IInputInjectorStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        interface IInputInjectorStatics2;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2 ABI::Windows::UI::Input::Preview::Injection::IInputInjectorStatics2

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        class InjectedInputKeyboardInfo;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("500e5efe-3bc1-5d9b-bcfc-c1f439505f12"))
IIterator<ABI::Windows::UI::Input::Preview::Injection::InjectedInputKeyboardInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Injection::InjectedInputKeyboardInfo*, ABI::Windows::UI::Input::Preview::Injection::IInjectedInputKeyboardInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Preview::Injection::InjectedInputKeyboardInfo*> __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_t;
#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("15d6330f-9c97-5705-b677-872585664fb5"))
IIterable<ABI::Windows::UI::Input::Preview::Injection::InjectedInputKeyboardInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Injection::InjectedInputKeyboardInfo*, ABI::Windows::UI::Input::Preview::Injection::IInjectedInputKeyboardInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Preview::Injection::InjectedInputKeyboardInfo*> __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_t;
#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        class InjectedInputMouseInfo;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9604d1d9-1744-5bd3-b5b9-d47b9434facb"))
IIterator<ABI::Windows::UI::Input::Preview::Injection::InjectedInputMouseInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Injection::InjectedInputMouseInfo*, ABI::Windows::UI::Input::Preview::Injection::IInjectedInputMouseInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Preview::Injection::InjectedInputMouseInfo*> __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_t;
#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6c34e5bd-0fa4-5244-89fb-04bfd480ecd8"))
IIterable<ABI::Windows::UI::Input::Preview::Injection::InjectedInputMouseInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Injection::InjectedInputMouseInfo*, ABI::Windows::UI::Input::Preview::Injection::IInjectedInputMouseInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Preview::Injection::InjectedInputMouseInfo*> __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_t;
#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        class InjectedInputTouchInfo;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4bc92e92-d32e-597a-ae24-b38861c5fb08"))
IIterator<ABI::Windows::UI::Input::Preview::Injection::InjectedInputTouchInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Injection::InjectedInputTouchInfo*, ABI::Windows::UI::Input::Preview::Injection::IInjectedInputTouchInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Preview::Injection::InjectedInputTouchInfo*> __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_t;
#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac5fac0b-82a0-5436-9284-e7db0bf4e615"))
IIterable<ABI::Windows::UI::Input::Preview::Injection::InjectedInputTouchInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Preview::Injection::InjectedInputTouchInfo*, ABI::Windows::UI::Input::Preview::Injection::IInjectedInputTouchInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Preview::Injection::InjectedInputTouchInfo*> __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_t;
#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef enum GamepadButtons : unsigned int GamepadButtons;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                typedef struct GamepadReading GamepadReading;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        typedef enum InjectedInputKeyOptions : unsigned int InjectedInputKeyOptions;
                    } /* Injection */
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
                    namespace Injection {
                        typedef enum InjectedInputMouseOptions : unsigned int InjectedInputMouseOptions;
                    } /* Injection */
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
                    namespace Injection {
                        typedef enum InjectedInputPenButtons : unsigned int InjectedInputPenButtons;
                    } /* Injection */
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
                    namespace Injection {
                        typedef enum InjectedInputPenParameters : unsigned int InjectedInputPenParameters;
                    } /* Injection */
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
                    namespace Injection {
                        typedef enum InjectedInputPointerOptions : unsigned int InjectedInputPointerOptions;
                    } /* Injection */
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
                    namespace Injection {
                        typedef enum InjectedInputShortcut : int InjectedInputShortcut;
                    } /* Injection */
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
                    namespace Injection {
                        typedef enum InjectedInputTouchParameters : unsigned int InjectedInputTouchParameters;
                    } /* Injection */
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
                    namespace Injection {
                        typedef enum InjectedInputVisualizationMode : int InjectedInputVisualizationMode;
                    } /* Injection */
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
                    namespace Injection {
                        typedef struct InjectedInputPoint InjectedInputPoint;
                    } /* Injection */
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
                    namespace Injection {
                        typedef struct InjectedInputPointerInfo InjectedInputPointerInfo;
                    } /* Injection */
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
                    namespace Injection {
                        typedef struct InjectedInputRectangle InjectedInputRectangle;
                    } /* Injection */
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
                    namespace Injection {
                        class InjectedInputGamepadInfo;
                    } /* Injection */
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
                    namespace Injection {
                        class InjectedInputPenInfo;
                    } /* Injection */
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
                    namespace Injection {
                        class InputInjector;
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputButtonChangeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputButtonChangeKind : int
                        {
                            InjectedInputButtonChangeKind_None = 0,
                            InjectedInputButtonChangeKind_FirstButtonDown = 1,
                            InjectedInputButtonChangeKind_FirstButtonUp = 2,
                            InjectedInputButtonChangeKind_SecondButtonDown = 3,
                            InjectedInputButtonChangeKind_SecondButtonUp = 4,
                            InjectedInputButtonChangeKind_ThirdButtonDown = 5,
                            InjectedInputButtonChangeKind_ThirdButtonUp = 6,
                            InjectedInputButtonChangeKind_FourthButtonDown = 7,
                            InjectedInputButtonChangeKind_FourthButtonUp = 8,
                            InjectedInputButtonChangeKind_FifthButtonDown = 9,
                            InjectedInputButtonChangeKind_FifthButtonUp = 10,
                        };
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputKeyOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputKeyOptions : unsigned int
                        {
                            InjectedInputKeyOptions_None = 0,
                            InjectedInputKeyOptions_ExtendedKey = 0x1,
                            InjectedInputKeyOptions_KeyUp = 0x2,
                            InjectedInputKeyOptions_ScanCode = 0x8,
                            InjectedInputKeyOptions_Unicode = 0x4,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(InjectedInputKeyOptions)
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputMouseOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputMouseOptions : unsigned int
                        {
                            InjectedInputMouseOptions_None = 0,
                            InjectedInputMouseOptions_Move = 0x1,
                            InjectedInputMouseOptions_LeftDown = 0x2,
                            InjectedInputMouseOptions_LeftUp = 0x4,
                            InjectedInputMouseOptions_RightDown = 0x8,
                            InjectedInputMouseOptions_RightUp = 0x10,
                            InjectedInputMouseOptions_MiddleDown = 0x20,
                            InjectedInputMouseOptions_MiddleUp = 0x40,
                            InjectedInputMouseOptions_XDown = 0x80,
                            InjectedInputMouseOptions_XUp = 0x100,
                            InjectedInputMouseOptions_Wheel = 0x800,
                            InjectedInputMouseOptions_HWheel = 0x1000,
                            InjectedInputMouseOptions_MoveNoCoalesce = 0x2000,
                            InjectedInputMouseOptions_VirtualDesk = 0x4000,
                            InjectedInputMouseOptions_Absolute = 0x8000,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(InjectedInputMouseOptions)
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPenButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputPenButtons : unsigned int
                        {
                            InjectedInputPenButtons_None = 0,
                            InjectedInputPenButtons_Barrel = 0x1,
                            InjectedInputPenButtons_Inverted = 0x2,
                            InjectedInputPenButtons_Eraser = 0x4,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(InjectedInputPenButtons)
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPenParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputPenParameters : unsigned int
                        {
                            InjectedInputPenParameters_None = 0,
                            InjectedInputPenParameters_Pressure = 0x1,
                            InjectedInputPenParameters_Rotation = 0x2,
                            InjectedInputPenParameters_TiltX = 0x4,
                            InjectedInputPenParameters_TiltY = 0x8,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(InjectedInputPenParameters)
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputPointerOptions : unsigned int
                        {
                            InjectedInputPointerOptions_None = 0,
                            InjectedInputPointerOptions_New = 0x1,
                            InjectedInputPointerOptions_InRange = 0x2,
                            InjectedInputPointerOptions_InContact = 0x4,
                            InjectedInputPointerOptions_FirstButton = 0x10,
                            InjectedInputPointerOptions_SecondButton = 0x20,
                            InjectedInputPointerOptions_Primary = 0x2000,
                            InjectedInputPointerOptions_Confidence = 0x4000,
                            InjectedInputPointerOptions_Canceled = 0x8000,
                            InjectedInputPointerOptions_PointerDown = 0x10000,
                            InjectedInputPointerOptions_Update = 0x20000,
                            InjectedInputPointerOptions_PointerUp = 0x40000,
                            InjectedInputPointerOptions_CaptureChanged = 0x200000,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(InjectedInputPointerOptions)
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputShortcut
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputShortcut : int
                        {
                            InjectedInputShortcut_Back = 0,
                            InjectedInputShortcut_Start = 1,
                            InjectedInputShortcut_Search = 2,
                        };
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputTouchParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputTouchParameters : unsigned int
                        {
                            InjectedInputTouchParameters_None = 0,
                            InjectedInputTouchParameters_Contact = 0x1,
                            InjectedInputTouchParameters_Orientation = 0x2,
                            InjectedInputTouchParameters_Pressure = 0x4,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(InjectedInputTouchParameters)
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputVisualizationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        enum InjectedInputVisualizationMode : int
                        {
                            InjectedInputVisualizationMode_None = 0,
                            InjectedInputVisualizationMode_Default = 1,
                            InjectedInputVisualizationMode_Indirect = 2,
                        };
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        struct InjectedInputPoint
                        {
                            INT32 PositionX;
                            INT32 PositionY;
                        };
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPointerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        struct InjectedInputPointerInfo
                        {
                            UINT32 PointerId;
                            ABI::Windows::UI::Input::Preview::Injection::InjectedInputPointerOptions PointerOptions;
                            ABI::Windows::UI::Input::Preview::Injection::InjectedInputPoint PixelLocation;
                            UINT32 TimeOffsetInMilliseconds;
                            UINT64 PerformanceCount;
                        };
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputRectangle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        struct InjectedInputRectangle
                        {
                            INT32 Left;
                            INT32 Top;
                            INT32 Bottom;
                            INT32 Right;
                        };
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputGamepadInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfo";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("20ae9a3f-df11-4572-a9ab-d75b8a5e48ad")
                        IInjectedInputGamepadInfo : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Buttons(
                                ABI::Windows::Gaming::Input::GamepadButtons* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Buttons(
                                ABI::Windows::Gaming::Input::GamepadButtons value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_LeftThumbstickX(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_LeftThumbstickX(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_LeftThumbstickY(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_LeftThumbstickY(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_LeftTrigger(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_LeftTrigger(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RightThumbstickX(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_RightThumbstickX(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RightThumbstickY(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_RightThumbstickY(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RightTrigger(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_RightTrigger(
                                DOUBLE value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInjectedInputGamepadInfo = __uuidof(IInjectedInputGamepadInfo);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputGamepadInfoFactory[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("59596876-6c39-4ec4-8b2a-29ef7de18aca")
                        IInjectedInputGamepadInfoFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromGamepadReading(
                                ABI::Windows::Gaming::Input::GamepadReading reading,
                                ABI::Windows::UI::Input::Preview::Injection::IInjectedInputGamepadInfo** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInjectedInputGamepadInfoFactory = __uuidof(IInjectedInputGamepadInfoFactory);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputKeyboardInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputKeyboardInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputKeyboardInfo";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("4b46d140-2b6a-5ffa-7eae-bd077b052acd")
                        IInjectedInputKeyboardInfo : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_KeyOptions(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputKeyOptions* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_KeyOptions(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputKeyOptions value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ScanCode(
                                UINT16* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_ScanCode(
                                UINT16 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_VirtualKey(
                                UINT16* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_VirtualKey(
                                UINT16 value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInjectedInputKeyboardInfo = __uuidof(IInjectedInputKeyboardInfo);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputMouseInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputMouseInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputMouseInfo";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("96f56e6b-e47a-5cf4-418d-8a5fb9670c7d")
                        IInjectedInputMouseInfo : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_MouseOptions(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputMouseOptions* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_MouseOptions(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputMouseOptions value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_MouseData(
                                UINT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_MouseData(
                                UINT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DeltaY(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_DeltaY(
                                INT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DeltaX(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_DeltaX(
                                INT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_TimeOffsetInMilliseconds(
                                UINT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_TimeOffsetInMilliseconds(
                                UINT32 value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInjectedInputMouseInfo = __uuidof(IInjectedInputMouseInfo);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputPenInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputPenInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputPenInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputPenInfo";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("6b40ad03-ca1e-5527-7e02-2828540bb1d4")
                        IInjectedInputPenInfo : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PointerInfo(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputPointerInfo* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PointerInfo(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputPointerInfo value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PenButtons(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputPenButtons* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PenButtons(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputPenButtons value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PenParameters(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputPenParameters* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PenParameters(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputPenParameters value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Pressure(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Pressure(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Rotation(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Rotation(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_TiltX(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_TiltX(
                                INT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_TiltY(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_TiltY(
                                INT32 value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInjectedInputPenInfo = __uuidof(IInjectedInputPenInfo);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputTouchInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputTouchInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputTouchInfo";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("224fd1df-43e8-5ef5-510a-69ca8c9b4c28")
                        IInjectedInputTouchInfo : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Contact(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputRectangle* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Contact(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputRectangle value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Orientation(
                                INT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PointerInfo(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputPointerInfo* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PointerInfo(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputPointerInfo value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Pressure(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Pressure(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_TouchParameters(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputTouchParameters* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_TouchParameters(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputTouchParameters value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInjectedInputTouchInfo = __uuidof(IInjectedInputTouchInfo);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInputInjector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InputInjector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInputInjector[] = L"Windows.UI.Input.Preview.Injection.IInputInjector";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("8ec26f84-0b02-4bd2-ad7a-3d4658be3e18")
                        IInputInjector : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE InjectKeyboardInput(
                                __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* input
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InjectMouseInput(
                                __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* input
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InitializeTouchInjection(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputVisualizationMode visualMode
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InjectTouchInput(
                                __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* input
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE UninitializeTouchInjection(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InitializePenInjection(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputVisualizationMode visualMode
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InjectPenInput(
                                ABI::Windows::UI::Input::Preview::Injection::IInjectedInputPenInfo* input
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE UninitializePenInjection(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InjectShortcut(
                                ABI::Windows::UI::Input::Preview::Injection::InjectedInputShortcut shortcut
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInputInjector = __uuidof(IInputInjector);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInputInjector2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InputInjector
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Preview.Injection.IInputInjector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInputInjector2[] = L"Windows.UI.Input.Preview.Injection.IInputInjector2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("8e7a905d-1453-43a7-9bcb-06d6d7b305f7")
                        IInputInjector2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE InitializeGamepadInjection(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InjectGamepadInput(
                                ABI::Windows::UI::Input::Preview::Injection::IInjectedInputGamepadInfo* input
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE UninitializeGamepadInjection(void) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInputInjector2 = __uuidof(IInputInjector2);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInputInjectorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InputInjector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInputInjectorStatics[] = L"Windows.UI.Input.Preview.Injection.IInputInjectorStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("deae6943-7402-4141-a5c6-0c01aa57b16a")
                        IInputInjectorStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE TryCreate(
                                ABI::Windows::UI::Input::Preview::Injection::IInputInjector** instance
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInputInjectorStatics = __uuidof(IInputInjectorStatics);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInputInjectorStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InputInjector
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Preview.Injection.IInputInjectorStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInputInjectorStatics2[] = L"Windows.UI.Input.Preview.Injection.IInputInjectorStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Preview {
                    namespace Injection {
                        MIDL_INTERFACE("a4db38fb-dd8c-414f-95ea-f87ef4c0ae6c")
                        IInputInjectorStatics2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE TryCreateForAppBroadcastOnly(
                                ABI::Windows::UI::Input::Preview::Injection::IInputInjector** instance
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IInputInjectorStatics2 = __uuidof(IInputInjectorStatics2);
                    } /* Injection */
                } /* Preview */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputGamepadInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputGamepadInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputGamepadInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputKeyboardInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputKeyboardInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputKeyboardInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputKeyboardInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputMouseInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputMouseInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputMouseInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputMouseInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputPenInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputPenInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputPenInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputPenInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputPenInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputPenInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputTouchInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputTouchInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputTouchInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputTouchInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InputInjector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Preview.Injection.IInputInjectorStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Input.Preview.Injection.IInputInjectorStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInputInjector ** Default Interface **
 *    Windows.UI.Input.Preview.Injection.IInputInjector2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InputInjector_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InputInjector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InputInjector[] = L"Windows.UI.Input.Preview.Injection.InputInjector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2 __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2 __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2;

#endif // ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo;

typedef struct __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfoVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo;

typedef struct __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* This,
        __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfoVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo;

typedef struct __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfoVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo;

typedef struct __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* This,
        __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfoVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo;

typedef struct __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfoVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo;

typedef struct __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* This,
        __FIIterator_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfoVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CGaming_CInput_CGamepadButtons __x_ABI_CWindows_CGaming_CInput_CGamepadButtons;

typedef struct __x_ABI_CWindows_CGaming_CInput_CGamepadReading __x_ABI_CWindows_CGaming_CInput_CGamepadReading;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputKeyOptions __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputKeyOptions;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputMouseOptions __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputMouseOptions;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenButtons __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenButtons;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenParameters __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenParameters;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerOptions __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerOptions;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputShortcut __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputShortcut;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputTouchParameters __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputTouchParameters;

typedef enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputVisualizationMode __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputVisualizationMode;

typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPoint __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPoint;

typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerInfo __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerInfo;

typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputRectangle __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputRectangle;

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputButtonChangeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputButtonChangeKind
{
    InjectedInputButtonChangeKind_None = 0,
    InjectedInputButtonChangeKind_FirstButtonDown = 1,
    InjectedInputButtonChangeKind_FirstButtonUp = 2,
    InjectedInputButtonChangeKind_SecondButtonDown = 3,
    InjectedInputButtonChangeKind_SecondButtonUp = 4,
    InjectedInputButtonChangeKind_ThirdButtonDown = 5,
    InjectedInputButtonChangeKind_ThirdButtonUp = 6,
    InjectedInputButtonChangeKind_FourthButtonDown = 7,
    InjectedInputButtonChangeKind_FourthButtonUp = 8,
    InjectedInputButtonChangeKind_FifthButtonDown = 9,
    InjectedInputButtonChangeKind_FifthButtonUp = 10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputKeyOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputKeyOptions
{
    InjectedInputKeyOptions_None = 0,
    InjectedInputKeyOptions_ExtendedKey = 0x1,
    InjectedInputKeyOptions_KeyUp = 0x2,
    InjectedInputKeyOptions_ScanCode = 0x8,
    InjectedInputKeyOptions_Unicode = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputMouseOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputMouseOptions
{
    InjectedInputMouseOptions_None = 0,
    InjectedInputMouseOptions_Move = 0x1,
    InjectedInputMouseOptions_LeftDown = 0x2,
    InjectedInputMouseOptions_LeftUp = 0x4,
    InjectedInputMouseOptions_RightDown = 0x8,
    InjectedInputMouseOptions_RightUp = 0x10,
    InjectedInputMouseOptions_MiddleDown = 0x20,
    InjectedInputMouseOptions_MiddleUp = 0x40,
    InjectedInputMouseOptions_XDown = 0x80,
    InjectedInputMouseOptions_XUp = 0x100,
    InjectedInputMouseOptions_Wheel = 0x800,
    InjectedInputMouseOptions_HWheel = 0x1000,
    InjectedInputMouseOptions_MoveNoCoalesce = 0x2000,
    InjectedInputMouseOptions_VirtualDesk = 0x4000,
    InjectedInputMouseOptions_Absolute = 0x8000,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPenButtons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenButtons
{
    InjectedInputPenButtons_None = 0,
    InjectedInputPenButtons_Barrel = 0x1,
    InjectedInputPenButtons_Inverted = 0x2,
    InjectedInputPenButtons_Eraser = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPenParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenParameters
{
    InjectedInputPenParameters_None = 0,
    InjectedInputPenParameters_Pressure = 0x1,
    InjectedInputPenParameters_Rotation = 0x2,
    InjectedInputPenParameters_TiltX = 0x4,
    InjectedInputPenParameters_TiltY = 0x8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerOptions
{
    InjectedInputPointerOptions_None = 0,
    InjectedInputPointerOptions_New = 0x1,
    InjectedInputPointerOptions_InRange = 0x2,
    InjectedInputPointerOptions_InContact = 0x4,
    InjectedInputPointerOptions_FirstButton = 0x10,
    InjectedInputPointerOptions_SecondButton = 0x20,
    InjectedInputPointerOptions_Primary = 0x2000,
    InjectedInputPointerOptions_Confidence = 0x4000,
    InjectedInputPointerOptions_Canceled = 0x8000,
    InjectedInputPointerOptions_PointerDown = 0x10000,
    InjectedInputPointerOptions_Update = 0x20000,
    InjectedInputPointerOptions_PointerUp = 0x40000,
    InjectedInputPointerOptions_CaptureChanged = 0x200000,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputShortcut
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputShortcut
{
    InjectedInputShortcut_Back = 0,
    InjectedInputShortcut_Start = 1,
    InjectedInputShortcut_Search = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputTouchParameters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputTouchParameters
{
    InjectedInputTouchParameters_None = 0,
    InjectedInputTouchParameters_Contact = 0x1,
    InjectedInputTouchParameters_Orientation = 0x2,
    InjectedInputTouchParameters_Pressure = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputVisualizationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputVisualizationMode
{
    InjectedInputVisualizationMode_None = 0,
    InjectedInputVisualizationMode_Default = 1,
    InjectedInputVisualizationMode_Indirect = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPoint
{
    INT32 PositionX;
    INT32 PositionY;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputPointerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerInfo
{
    UINT32 PointerId;
    enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerOptions PointerOptions;
    struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPoint PixelLocation;
    UINT32 TimeOffsetInMilliseconds;
    UINT64 PerformanceCount;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Preview.Injection.InjectedInputRectangle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputRectangle
{
    INT32 Left;
    INT32 Top;
    INT32 Bottom;
    INT32 Right;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputGamepadInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfo";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Buttons)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        enum __x_ABI_CWindows_CGaming_CInput_CGamepadButtons* value);
    HRESULT (STDMETHODCALLTYPE* put_Buttons)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        enum __x_ABI_CWindows_CGaming_CInput_CGamepadButtons value);
    HRESULT (STDMETHODCALLTYPE* get_LeftThumbstickX)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_LeftThumbstickX)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_LeftThumbstickY)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_LeftThumbstickY)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_LeftTrigger)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_LeftTrigger)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RightThumbstickX)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RightThumbstickX)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RightThumbstickY)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RightThumbstickY)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RightTrigger)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RightTrigger)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_get_Buttons(This, value) \
    ((This)->lpVtbl->get_Buttons(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_put_Buttons(This, value) \
    ((This)->lpVtbl->put_Buttons(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_get_LeftThumbstickX(This, value) \
    ((This)->lpVtbl->get_LeftThumbstickX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_put_LeftThumbstickX(This, value) \
    ((This)->lpVtbl->put_LeftThumbstickX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_get_LeftThumbstickY(This, value) \
    ((This)->lpVtbl->get_LeftThumbstickY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_put_LeftThumbstickY(This, value) \
    ((This)->lpVtbl->put_LeftThumbstickY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_get_LeftTrigger(This, value) \
    ((This)->lpVtbl->get_LeftTrigger(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_put_LeftTrigger(This, value) \
    ((This)->lpVtbl->put_LeftTrigger(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_get_RightThumbstickX(This, value) \
    ((This)->lpVtbl->get_RightThumbstickX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_put_RightThumbstickX(This, value) \
    ((This)->lpVtbl->put_RightThumbstickX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_get_RightThumbstickY(This, value) \
    ((This)->lpVtbl->get_RightThumbstickY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_put_RightThumbstickY(This, value) \
    ((This)->lpVtbl->put_RightThumbstickY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_get_RightTrigger(This, value) \
    ((This)->lpVtbl->get_RightTrigger(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_put_RightTrigger(This, value) \
    ((This)->lpVtbl->put_RightTrigger(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputGamepadInfoFactory[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceFromGamepadReading)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory* This,
        struct __x_ABI_CWindows_CGaming_CInput_CGamepadReading reading,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactoryVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_CreateInstanceFromGamepadReading(This, reading, value) \
    ((This)->lpVtbl->CreateInstanceFromGamepadReading(This, reading, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputKeyboardInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputKeyboardInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputKeyboardInfo";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KeyOptions)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputKeyOptions* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyOptions)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputKeyOptions value);
    HRESULT (STDMETHODCALLTYPE* get_ScanCode)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* put_ScanCode)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        UINT16 value);
    HRESULT (STDMETHODCALLTYPE* get_VirtualKey)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* put_VirtualKey)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo* This,
        UINT16 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfoVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_get_KeyOptions(This, value) \
    ((This)->lpVtbl->get_KeyOptions(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_put_KeyOptions(This, value) \
    ((This)->lpVtbl->put_KeyOptions(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_get_ScanCode(This, value) \
    ((This)->lpVtbl->get_ScanCode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_put_ScanCode(This, value) \
    ((This)->lpVtbl->put_ScanCode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_get_VirtualKey(This, value) \
    ((This)->lpVtbl->get_VirtualKey(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_put_VirtualKey(This, value) \
    ((This)->lpVtbl->put_VirtualKey(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputKeyboardInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputMouseInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputMouseInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputMouseInfo";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MouseOptions)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputMouseOptions* value);
    HRESULT (STDMETHODCALLTYPE* put_MouseOptions)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputMouseOptions value);
    HRESULT (STDMETHODCALLTYPE* get_MouseData)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MouseData)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DeltaY)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DeltaY)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DeltaX)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DeltaX)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TimeOffsetInMilliseconds)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TimeOffsetInMilliseconds)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfoVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_get_MouseOptions(This, value) \
    ((This)->lpVtbl->get_MouseOptions(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_put_MouseOptions(This, value) \
    ((This)->lpVtbl->put_MouseOptions(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_get_MouseData(This, value) \
    ((This)->lpVtbl->get_MouseData(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_put_MouseData(This, value) \
    ((This)->lpVtbl->put_MouseData(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_get_DeltaY(This, value) \
    ((This)->lpVtbl->get_DeltaY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_put_DeltaY(This, value) \
    ((This)->lpVtbl->put_DeltaY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_get_DeltaX(This, value) \
    ((This)->lpVtbl->get_DeltaX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_put_DeltaX(This, value) \
    ((This)->lpVtbl->put_DeltaX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_get_TimeOffsetInMilliseconds(This, value) \
    ((This)->lpVtbl->get_TimeOffsetInMilliseconds(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_put_TimeOffsetInMilliseconds(This, value) \
    ((This)->lpVtbl->put_TimeOffsetInMilliseconds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputMouseInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputPenInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputPenInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputPenInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputPenInfo";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerInfo)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerInfo* value);
    HRESULT (STDMETHODCALLTYPE* put_PointerInfo)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerInfo value);
    HRESULT (STDMETHODCALLTYPE* get_PenButtons)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenButtons* value);
    HRESULT (STDMETHODCALLTYPE* put_PenButtons)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenButtons value);
    HRESULT (STDMETHODCALLTYPE* get_PenParameters)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenParameters* value);
    HRESULT (STDMETHODCALLTYPE* put_PenParameters)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPenParameters value);
    HRESULT (STDMETHODCALLTYPE* get_Pressure)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Pressure)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Rotation)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Rotation)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_TiltX)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TiltX)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TiltY)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TiltY)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfoVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_get_PointerInfo(This, value) \
    ((This)->lpVtbl->get_PointerInfo(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_put_PointerInfo(This, value) \
    ((This)->lpVtbl->put_PointerInfo(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_get_PenButtons(This, value) \
    ((This)->lpVtbl->get_PenButtons(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_put_PenButtons(This, value) \
    ((This)->lpVtbl->put_PenButtons(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_get_PenParameters(This, value) \
    ((This)->lpVtbl->get_PenParameters(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_put_PenParameters(This, value) \
    ((This)->lpVtbl->put_PenParameters(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_get_Pressure(This, value) \
    ((This)->lpVtbl->get_Pressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_put_Pressure(This, value) \
    ((This)->lpVtbl->put_Pressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_get_Rotation(This, value) \
    ((This)->lpVtbl->get_Rotation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_put_Rotation(This, value) \
    ((This)->lpVtbl->put_Rotation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_get_TiltX(This, value) \
    ((This)->lpVtbl->get_TiltX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_put_TiltX(This, value) \
    ((This)->lpVtbl->put_TiltX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_get_TiltY(This, value) \
    ((This)->lpVtbl->get_TiltY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_put_TiltY(This, value) \
    ((This)->lpVtbl->put_TiltY(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInjectedInputTouchInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInjectedInputTouchInfo[] = L"Windows.UI.Input.Preview.Injection.IInjectedInputTouchInfo";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputRectangle* value);
    HRESULT (STDMETHODCALLTYPE* put_Contact)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputRectangle value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Orientation)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_PointerInfo)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerInfo* value);
    HRESULT (STDMETHODCALLTYPE* put_PointerInfo)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputPointerInfo value);
    HRESULT (STDMETHODCALLTYPE* get_Pressure)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Pressure)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_TouchParameters)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputTouchParameters* value);
    HRESULT (STDMETHODCALLTYPE* put_TouchParameters)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputTouchParameters value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfoVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_put_Contact(This, value) \
    ((This)->lpVtbl->put_Contact(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_put_Orientation(This, value) \
    ((This)->lpVtbl->put_Orientation(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_get_PointerInfo(This, value) \
    ((This)->lpVtbl->get_PointerInfo(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_put_PointerInfo(This, value) \
    ((This)->lpVtbl->put_PointerInfo(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_get_Pressure(This, value) \
    ((This)->lpVtbl->get_Pressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_put_Pressure(This, value) \
    ((This)->lpVtbl->put_Pressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_get_TouchParameters(This, value) \
    ((This)->lpVtbl->get_TouchParameters(This, value))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_put_TouchParameters(This, value) \
    ((This)->lpVtbl->put_TouchParameters(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputTouchInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInputInjector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InputInjector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInputInjector[] = L"Windows.UI.Input.Preview.Injection.IInputInjector";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* InjectKeyboardInput)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputKeyboardInfo* input);
    HRESULT (STDMETHODCALLTYPE* InjectMouseInput)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputMouseInfo* input);
    HRESULT (STDMETHODCALLTYPE* InitializeTouchInjection)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputVisualizationMode visualMode);
    HRESULT (STDMETHODCALLTYPE* InjectTouchInput)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        __FIIterable_1_Windows__CUI__CInput__CPreview__CInjection__CInjectedInputTouchInfo* input);
    HRESULT (STDMETHODCALLTYPE* UninitializeTouchInjection)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This);
    HRESULT (STDMETHODCALLTYPE* InitializePenInjection)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputVisualizationMode visualMode);
    HRESULT (STDMETHODCALLTYPE* InjectPenInput)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputPenInfo* input);
    HRESULT (STDMETHODCALLTYPE* UninitializePenInjection)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This);
    HRESULT (STDMETHODCALLTYPE* InjectShortcut)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector* This,
        enum __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CInjectedInputShortcut shortcut);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_InjectKeyboardInput(This, input) \
    ((This)->lpVtbl->InjectKeyboardInput(This, input))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_InjectMouseInput(This, input) \
    ((This)->lpVtbl->InjectMouseInput(This, input))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_InitializeTouchInjection(This, visualMode) \
    ((This)->lpVtbl->InitializeTouchInjection(This, visualMode))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_InjectTouchInput(This, input) \
    ((This)->lpVtbl->InjectTouchInput(This, input))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_UninitializeTouchInjection(This) \
    ((This)->lpVtbl->UninitializeTouchInjection(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_InitializePenInjection(This, visualMode) \
    ((This)->lpVtbl->InitializePenInjection(This, visualMode))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_InjectPenInput(This, input) \
    ((This)->lpVtbl->InjectPenInput(This, input))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_UninitializePenInjection(This) \
    ((This)->lpVtbl->UninitializePenInjection(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_InjectShortcut(This, shortcut) \
    ((This)->lpVtbl->InjectShortcut(This, shortcut))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInputInjector2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InputInjector
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Preview.Injection.IInputInjector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInputInjector2[] = L"Windows.UI.Input.Preview.Injection.IInputInjector2";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* InitializeGamepadInjection)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This);
    HRESULT (STDMETHODCALLTYPE* InjectGamepadInput)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInjectedInputGamepadInfo* input);
    HRESULT (STDMETHODCALLTYPE* UninitializeGamepadInjection)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_InitializeGamepadInjection(This) \
    ((This)->lpVtbl->InitializeGamepadInjection(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_InjectGamepadInput(This, input) \
    ((This)->lpVtbl->InjectGamepadInput(This, input))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_UninitializeGamepadInjection(This) \
    ((This)->lpVtbl->UninitializeGamepadInjection(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInputInjectorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InputInjector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInputInjectorStatics[] = L"Windows.UI.Input.Preview.Injection.IInputInjectorStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCreate)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector** instance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_TryCreate(This, instance) \
    ((This)->lpVtbl->TryCreate(This, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Preview.Injection.IInputInjectorStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Preview.Injection.InputInjector
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Preview.Injection.IInputInjectorStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Preview_Injection_IInputInjectorStatics2[] = L"Windows.UI.Input.Preview.Injection.IInputInjectorStatics2";
typedef struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCreateForAppBroadcastOnly)(__x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2* This,
        __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjector** instance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_TryCreateForAppBroadcastOnly(This, instance) \
    ((This)->lpVtbl->TryCreateForAppBroadcastOnly(This, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CPreview_CInjection_CIInputInjectorStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfoFactory interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputGamepadInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputGamepadInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputGamepadInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputGamepadInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputKeyboardInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputKeyboardInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputKeyboardInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputKeyboardInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputMouseInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputMouseInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputMouseInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputMouseInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputPenInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputPenInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputPenInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputPenInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputPenInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputPenInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInjectedInputTouchInfo ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputTouchInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InjectedInputTouchInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InjectedInputTouchInfo[] = L"Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Preview.Injection.InputInjector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Preview.Injection.IInputInjectorStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Input.Preview.Injection.IInputInjectorStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Preview.Injection.IInputInjector ** Default Interface **
 *    Windows.UI.Input.Preview.Injection.IInputInjector2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InputInjector_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Preview_Injection_InputInjector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Preview_Injection_InputInjector[] = L"Windows.UI.Input.Preview.Injection.InputInjector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Einput2Epreview2Einjection_p_h__

#endif // __windows2Eui2Einput2Epreview2Einjection_h__
